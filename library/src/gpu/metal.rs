use boxer::boxes::ReferenceBox;
use boxer::{ReturnBoxerResult, ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use cocoa::base::YES;
use cocoa::{appkit::NSView, base::id as cocoa_id};
use compositor::{Compositor, Layer};
use compositor_skia::{Cache, SkiaCompositor};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use fps_counter::FPSCounter;
use metal::{CommandQueue, Device, MTLPixelFormat, MetalDrawableRef, MetalLayer};
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};
use skia_safe::{scalar, Color, Color4f, ColorType, Font, Paint, Point, Size, Surface, Typeface};
use std::mem;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct MetalCompositor {
    context: MetalContext,
    latest_frame: Mutex<Option<Arc<dyn Layer>>>,
    cache: Cache,
    render_fps: Option<FPSCounter>,
}

lazy_static! {
    static ref FPS_FONT: Font = Font::new(Typeface::default(), 60.0);
    static ref FPS_PAINT: Paint = Paint::new(Color4f::from(Color::BLUE), None);
}

impl MetalCompositor {
    pub fn new(ns_view: cocoa_id, size: Option<CGSize>) -> Self {
        Self {
            context: MetalContext::new(ns_view, size),
            latest_frame: Mutex::new(None),
            cache: Cache::new(),
            render_fps: None,
        }
    }

    /// Resize the surface we render on. Must only be called from the main thread
    pub fn resize_surface(&mut self, size: CGSize) {
        self.context.resize_surface(size);
    }

    /// Submit the new layer to be rendered next. Can be called from any thread
    pub fn submit_layer(&mut self, layer: Arc<dyn Layer>) {
        self.latest_frame.lock().unwrap().replace(layer);
    }

    pub fn enable_fps(&mut self) {
        self.render_fps.replace(FPSCounter::default());
    }

    pub fn disable_fps(&mut self) {
        self.render_fps.take();
    }

    pub fn draw(&mut self) {
        let current_layer = self.latest_frame.lock().unwrap().clone();

        if let Some(layer) = current_layer {
            if let Some(MetalSurface {
                mut surface,
                drawable,
            }) = self.context.new_surface()
            {
                let canvas = surface.canvas();
                canvas.clear(Color::WHITE);

                SkiaCompositor::new(canvas, &mut self.cache).compose(layer);

                self.render_fps.as_mut().map(|counter| {
                    canvas.draw_str(
                        &format!("{}", counter.tick()),
                        Point::new(20.0, 70.0),
                        &FPS_FONT,
                        &FPS_PAINT,
                    );
                });

                surface.flush_and_submit();
                drop(surface);

                self.context.commit(drawable.as_ref());
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MetalContext {
    device: Device,
    layer: MetalLayer,
    queue: CommandQueue,
    backend_context: BackendContext,
    direct_context: DirectContext,
}

#[derive(Debug)]
pub struct MetalSurface {
    surface: Surface,
    drawable: ReferenceBox<MetalDrawableRef>,
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

    pub fn resize_surface(&mut self, size: CGSize) {
        self.layer.set_drawable_size(size);
    }

    pub fn new_surface(&mut self) -> Option<MetalSurface> {
        self.layer.next_drawable().and_then(|drawable| {
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

            Surface::from_backend_render_target(
                &mut self.direct_context,
                &backend_render_target,
                SurfaceOrigin::TopLeft,
                ColorType::BGRA8888,
                None,
                None,
            )
            .map(|surface| MetalSurface {
                surface,
                drawable: ReferenceBox::new(unsafe { std::mem::transmute(drawable.as_ptr()) }),
            })
        })
    }

    pub fn commit(&self, drawable: &MetalDrawableRef) {
        let command_buffer = self.queue.new_command_buffer();
        command_buffer.present_drawable(drawable);
        command_buffer.commit()
    }
}

#[no_mangle]
pub fn skia_metal_compositor_new(ns_view: cocoa_id) -> *mut ValueBox<MetalCompositor> {
    ValueBox::new(MetalCompositor::new(ns_view, None)).into_raw()
}

#[no_mangle]
pub fn skia_metal_compositor_new_size(
    ns_view: cocoa_id,
    width: u32,
    height: u32,
) -> *mut ValueBox<MetalCompositor> {
    ValueBox::new(MetalCompositor::new(
        ns_view,
        Some(CGSize::new(width.into(), height.into())),
    ))
    .into_raw()
}

#[no_mangle]
pub fn skia_metal_compositor_submit_layer(
    compositor: *mut ValueBox<MetalCompositor>,
    layer: *mut ValueBox<Arc<dyn Layer>>,
) {
    compositor
        .to_ref()
        .and_then(|mut compositor| {
            layer
                .to_ref()
                .map(|layer| compositor.submit_layer(layer.deref().clone()))
        })
        .log();
}

#[no_mangle]
pub fn skia_metal_compositor_draw(compositor: *mut ValueBox<MetalCompositor>) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.draw())
        .log();
}

#[no_mangle]
pub fn skia_metal_compositor_resize(
    compositor: *mut ValueBox<MetalCompositor>,
    width: u32,
    height: u32,
) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.resize_surface(CGSize::new(width.into(), height.into())))
        .log();
}

#[no_mangle]
pub fn skia_metal_compositor_enable_fps(compositor: *mut ValueBox<MetalCompositor>) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.enable_fps())
        .log();
}

#[no_mangle]
pub fn skia_metal_compositor_disable_fps(compositor: *mut ValueBox<MetalCompositor>) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.disable_fps())
        .log();
}

#[no_mangle]
pub fn skia_metal_compositor_drop(compositor: &mut *mut ValueBox<MetalCompositor>) {
    compositor.drop();
}
