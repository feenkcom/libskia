use crate::gpu::platform_compositor::{PlatformCompositor, PlatformContext};
use cocoa::appkit::NSView;
use cocoa::base::{id as cocoa_id, YES};
use std::ffi::c_void;

use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{CommandQueue, Device, MTLPixelFormat, MetalDrawableRef, MetalLayer};
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};
use skia_safe::{scalar, ColorType, ISize, Size, Surface};
use std::mem;
use value_box::ValueBox;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MetalContext {
    device: Device,
    layer: MetalLayer,
    queue: CommandQueue,
    backend_context: BackendContext,
    direct_context: DirectContext,
}

impl MetalContext {
    pub fn new(ns_view: cocoa_id, size: Option<CGSize>) -> Self {
        let device = Device::system_default().expect("no device found");

        let layer = {
            let layer = MetalLayer::new();
            layer.set_device(&device);
            layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);
            if let Some(size) = size {
                layer.set_drawable_size(size);
            }

            unsafe {
                ns_view.setWantsLayer(YES);
                ns_view.setLayer(mem::transmute(layer.as_ref()));
            }
            layer
        };

        let queue = device.new_command_queue();

        let backend_context = unsafe {
            mtl::BackendContext::new(
                device.as_ptr() as mtl::Handle,
                queue.as_ptr() as mtl::Handle,
                std::ptr::null(),
            )
        };

        let direct_context = DirectContext::new_metal(&backend_context, None).unwrap();

        MetalContext {
            device,
            layer,
            queue,
            backend_context,
            direct_context,
        }
    }

    pub fn resize_surface(&mut self, size: ISize) {
        self.layer
            .set_drawable_size(CGSize::new(size.width.into(), size.height.into()));
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        if let Some(drawable) = self.layer.next_drawable() {
            let drawable_size = {
                let size = self.layer.drawable_size();
                Size::new(size.width as scalar, size.height as scalar)
            };

            let texture_info =
                unsafe { mtl::TextureInfo::new(drawable.texture().as_ptr() as mtl::Handle) };

            let backend_render_target = BackendRenderTarget::new_metal(
                (drawable_size.width as i32, drawable_size.height as i32),
                1,
                &texture_info,
            );

            if let Some(mut surface) = Surface::from_backend_render_target(
                &mut self.direct_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            ) {
                callback(&mut surface);

                surface.flush_and_submit();
                drop(surface);

                self.commit(drawable);
            };
        }
    }

    pub fn commit(&self, drawable: &MetalDrawableRef) {
        let command_buffer = self.queue.new_command_buffer();
        command_buffer.present_drawable(drawable);
        command_buffer.commit()
    }
}

#[no_mangle]
pub fn skia_metal_compositor_new_size(
    ns_view: *mut c_void,
    width: u32,
    height: u32,
) -> *mut ValueBox<PlatformCompositor> {
    ValueBox::new(PlatformCompositor::new(PlatformContext::Metal(
        MetalContext::new(
            ns_view as cocoa_id,
            Some(CGSize::new(width.into(), height.into())),
        ),
    )))
    .into_raw()
}
