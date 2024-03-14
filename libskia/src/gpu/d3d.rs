use std::mem::transmute;

use skia_safe::gpu::d3d::BackendContext;
use skia_safe::gpu::{
    BackendRenderTarget, BackendTexture, DirectContext, FlushInfo, Protected, SurfaceOrigin,
    SyncCpu,
};
use skia_safe::surface::BackendSurfaceAccess;
use skia_safe::{gpu, ColorType, ISize, Surface};
use value_box::ValueBox;
use windows::core::{Interface, Result};
use windows::Win32::Foundation::{HANDLE, HWND};
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM;
use windows::Win32::Graphics::Dxgi::*;
use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObjectEx};

use crate::gpu::platform_compositor::{PlatformCompositor, PlatformContext};

pub const NUM_FRAMES: usize = 2;

#[derive(Debug)]
pub struct D3D12HardwareContext {
    queue: ID3D12CommandQueue,
    device: ID3D12Device,
    swap_chain: IDXGISwapChain3,
    direct_context: DirectContext,
}

#[derive(Debug)]
pub struct D3D12SurfaceBuffer {
    surface: Surface,
    buffer: ID3D12Resource,
    fence_value: u64,
}

#[derive(Debug)]
pub struct D3D12Context {
    hardware_context: D3D12HardwareContext,
    buffer_index: usize,
    surface_buffers: Option<[D3D12SurfaceBuffer; NUM_FRAMES]>,
    fence: ID3D12Fence,
    fence_event: HANDLE,
}

impl D3D12Context {
    pub fn new(window: HWND, width: u32, height: u32) -> Self {
        let factory: IDXGIFactory4 = create_factory().expect("Creating DXGI factory");
        let adapter: IDXGIAdapter1 = Self::create_hardware_adapter(&factory);

        let device: ID3D12Device = resolve_interface(
            |ptr| unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, ptr) }
        )
        .expect("Creating D3D device");

        let queue: ID3D12CommandQueue = {
            let desc =
                D3D12_COMMAND_QUEUE_DESC {
                    Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                    Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
                    Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
                    NodeMask: 0,
                };

            unsafe { device.CreateCommandQueue(&desc) }.expect("Creating command queue")
        };

        let backend_context = {
            let skia_adapter: skia_safe::gpu::d3d::IDXGIAdapter1 =
                unsafe { transmute(adapter.clone()) };
            let skia_device: skia_safe::gpu::d3d::ID3D12Device =
                unsafe { transmute(device.clone()) };
            let skia_queue: skia_safe::gpu::d3d::ID3D12CommandQueue =
                unsafe { transmute(queue.clone()) };

            BackendContext {
                adapter: skia_adapter,
                device: skia_device,
                queue: skia_queue,
                memory_allocator: None,
                protected_context: Protected::No,
            }
        };

        let direct_context = unsafe { DirectContext::new_d3d(&backend_context, None) }.unwrap();

        let swap_chain: IDXGISwapChain3 = {
            let factory: IDXGIFactory3 = create_factory().expect("Creating DXGI factory");

            let mut swap_chain_desc = DXGI_SWAP_CHAIN_DESC1::default();
            swap_chain_desc.BufferCount = NUM_FRAMES as u32;
            swap_chain_desc.Width = width as _;
            swap_chain_desc.Height = height as _;
            swap_chain_desc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
            swap_chain_desc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
            swap_chain_desc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
            swap_chain_desc.SampleDesc.Count = 1;

            let swap_chain1: IDXGISwapChain1 = unsafe {
                factory.CreateSwapChainForHwnd(
                    &queue,
                    &window,
                    &swap_chain_desc,
                    std::ptr::null_mut(),
                    None,
                )
            }
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to create Swap chain for window {:?} with description {:?} due to {}",
                    window, &swap_chain_desc, e
                )
            });

            unsafe { transmute(swap_chain1) }
        };

        unsafe { factory.MakeWindowAssociation(&window, DXGI_MWA_NO_ALT_ENTER) }
            .expect("Prevent DXGI from responding to an alt-enter sequence");

        let mut hardware_context = D3D12HardwareContext {
            queue,
            device,
            swap_chain,
            direct_context,
        };

        let surface_buffers = hardware_context.create_surface_buffers(width, height);
        let buffer_index = hardware_context.get_current_back_buffer_index();
        let fence = hardware_context.create_fence(surface_buffers[buffer_index].fence_value);
        let fence_event: HANDLE = unsafe { CreateEventW(std::ptr::null(), false, false, None) };

        Self {
            hardware_context,
            buffer_index,
            surface_buffers: Some(surface_buffers),
            fence,
            fence_event,
        }
    }

    pub fn resize(&mut self, size: ISize) {
        self.hardware_context
            .direct_context
            .flush(&FlushInfo::default());
        self.hardware_context
            .direct_context
            .submit(Some(SyncCpu::Yes));
        self.hardware_context.direct_context.free_gpu_resources();

        for surface_buffer in self.surface_buffers.take().unwrap() {
            self.wait_for_completion_surface_buffer(surface_buffer);
        }

        unsafe {
            self.hardware_context
                .swap_chain
                .ResizeBuffers(
                    0,
                    size.width as _,
                    size.height as _,
                    DXGI_FORMAT_R8G8B8A8_UNORM,
                    0,
                )
                .expect(&format!(
                    "Resize buffers to width = {} height = {}",
                    size.width, size.height
                ));
        }

        self.surface_buffers = Some(
            self.hardware_context
                .create_surface_buffers(size.width as _, size.height as _),
        );
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        let surface = self.get_back_buffer_surface();
        callback(surface);
        self.swap_buffers();
    }

    pub fn get_back_buffer_surface(&mut self) -> &mut Surface {
        let current_fence_value = self.get_fence_value();
        self.buffer_index = self.hardware_context.get_current_back_buffer_index();

        if unsafe { self.fence.GetCompletedValue() } < self.get_fence_value() {
            unsafe {
                self.fence
                    .SetEventOnCompletion(self.get_fence_value(), self.fence_event)
                    .expect("Set event on completion");
                WaitForSingleObjectEx(self.fence_event, u32::max_value(), false);
            };
        }

        let surface_buffer = &mut self.surface_buffers.as_mut().unwrap()[self.buffer_index];
        surface_buffer.fence_value = current_fence_value + 1;
        &mut surface_buffer.surface
    }

    pub fn swap_buffers(&mut self) {
        let surface = &mut self.surface_buffers.as_mut().unwrap()[self.buffer_index].surface;
        let flush_info = skia_safe::gpu::FlushInfo::default();
        self.hardware_context
            .direct_context
            .flush_surface_with_access(surface, BackendSurfaceAccess::Present, &flush_info);
        self.hardware_context.direct_context.submit(None);

        unsafe {
            self.hardware_context
                .device
                .GetDeviceRemovedReason()
                .expect("Device is not removed");
            self.hardware_context
                .swap_chain
                .Present(1, 0)
                .expect("Present swap chain");
            self.hardware_context
                .queue
                .Signal(&self.fence, self.get_fence_value())
                .expect("Signal queue");
        }
    }

    fn wait_for_completion_surface_buffer(&self, surface_buffer: D3D12SurfaceBuffer) {
        if unsafe { self.fence.GetCompletedValue() } < surface_buffer.fence_value {
            unsafe {
                self.fence
                    .SetEventOnCompletion(surface_buffer.fence_value, self.fence_event)
                    .expect("Set event on completion");
                WaitForSingleObjectEx(self.fence_event, u32::max_value(), false);
            };
        }

        drop(surface_buffer.surface);
        drop(surface_buffer.buffer);
    }

    fn create_hardware_adapter(factory: &IDXGIFactory4) -> IDXGIAdapter1 {
        let mut index = 0;

        loop {
            match unsafe { factory.EnumAdapters1(index) } {
                Ok(adapter) => {
                    let device: Result<ID3D12Device> =
                        resolve_interface(|ptr| unsafe {
                            D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, ptr)
                        });

                    if device.is_ok() {
                        return adapter;
                    }
                }
                Err(err) => {
                    panic!("Failed to create d3d adapter {}", err);
                }
            }
            index = index + 1;
        }
    }

    fn get_fence_value(&self) -> u64 {
        self.surface_buffers.as_ref().unwrap()[self.buffer_index].fence_value
    }
}

impl D3D12HardwareContext {
    fn get_current_back_buffer_index(&self) -> usize {
        unsafe { self.swap_chain.GetCurrentBackBufferIndex() as usize }
    }

    fn create_fence(&self, fence_value: u64) -> ID3D12Fence {
        unsafe { self.device.CreateFence(fence_value, D3D12_FENCE_FLAG_NONE) }
            .expect("Create fence")
    }

    fn create_surface_buffers(
        &mut self,
        width: u32,
        height: u32,
    ) -> [D3D12SurfaceBuffer; NUM_FRAMES] {
        [
            self.create_surface_buffer(0, width, height),
            self.create_surface_buffer(1, width, height),
        ]
    }

    fn create_surface_buffer(&mut self, index: u32, width: u32, height: u32) -> D3D12SurfaceBuffer {
        let buffer: ID3D12Resource =
            unsafe { self.swap_chain.GetBuffer(index) }.expect(&format!("Get buffer {}", index));

        let texture_info = skia_safe::gpu::d3d::TextureResourceInfo {
            resource: unsafe { transmute(buffer.clone()) },
            alloc: None,
            resource_state: D3D12_RESOURCE_STATE_PRESENT.0,
            format: DXGI_FORMAT_R8G8B8A8_UNORM.0,
            sample_count: 1,
            level_count: 1,
            sample_quality_pattern: 0,
            protected: Protected::No,
        };

        let surface = if true {
            let backend_texture =
                BackendTexture::new_d3d((width as i32, height as i32), &texture_info);
            gpu::surfaces::wrap_backend_texture(
                &mut self.direct_context,
                &backend_texture,
                SurfaceOrigin::TopLeft,
                1,
                ColorType::RGBA8888,
                None,
                None,
            )
            .expect("Create surface")
        } else {
            let backend_render_target =
                BackendRenderTarget::new_d3d((width as i32, height as i32), &texture_info);

            gpu::surfaces::wrap_backend_render_target(
                &mut self.direct_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::RGBA8888,
                None,
                None,
            )
            .expect("Create surface")
        };

        D3D12SurfaceBuffer {
            surface,
            buffer,
            fence_value: 10000,
        }
    }
}

fn resolve_interface<T: Interface>(f: impl FnOnce(&mut Option<T>) -> Result<()>) -> Result<T> {
    let mut res: Option<T> = None;
    f(&mut res)?;
    Ok(res.unwrap())
}

fn create_factory<T: Interface>() -> Result<T> {
    let factory: Result<T> = unsafe { CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG) };
    factory.or_else(|error| {
        warn!(
            "Could not create Debug factory: {:?}. We will try a default one.",
            error
        );
        unsafe { CreateDXGIFactory2(0) }
    })
}

#[no_mangle]
pub fn skia_d3d_compositor_new_size(
    window: HWND,
    width: u32,
    height: u32,
) -> *mut ValueBox<PlatformCompositor> {
    ValueBox::new(
        PlatformCompositor::new(PlatformContext::D3D(D3D12Context::new(window, width, height)))
    )
    .into_raw()
}
