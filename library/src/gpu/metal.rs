use boxer::boxes::ReferenceBox;
use boxer::boxes::ReferenceBoxPointer;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use cocoa::base::YES;
use cocoa::{appkit::NSView, base::id as cocoa_id};
use compositor::{Compositor, Layer};
use compositor_skia::{ImageCache, ShadowCache, SkiaCompositor};
use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{CommandQueue, Device, MTLPixelFormat, MetalDrawableRef, MetalLayer};
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, BackendRenderTarget, ContextOptions, DirectContext, SurfaceOrigin};
use skia_safe::{scalar, ColorType, Size, Surface};
use std::mem;
use std::sync::Arc;

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
pub fn skia_context_new_metal(ns_view: cocoa_id) -> *mut ValueBox<MetalContext> {
    ValueBox::new(MetalContext::new(ns_view, None)).into_raw()
}

#[no_mangle]
pub fn skia_context_new_metal_size(
    ns_view: cocoa_id,
    width: f32,
    height: f32,
) -> *mut ValueBox<MetalContext> {
    ValueBox::new(MetalContext::new(
        ns_view,
        Some(CGSize::new(width.into(), height.into())),
    ))
    .into_raw()
}

#[no_mangle]
pub fn skia_metal_context_new_metal_surface(
    metal_context: *mut ValueBox<MetalContext>,
) -> *mut ValueBox<MetalSurface> {
    metal_context.with_not_null_return(std::ptr::null_mut(), |metal_context| {
        match metal_context.new_surface() {
            None => std::ptr::null_mut(),
            Some(surface) => ValueBox::new(surface).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_metal_context_draw_composition_layer(
    context: *mut ValueBox<MetalContext>,
    layer: *mut ValueBox<Arc<dyn Layer>>,
    image_cache: *mut ValueBox<ImageCache>,
    shadow_cache: *mut ValueBox<ShadowCache>,
) {
    context.with_not_null(|metal_context| {
        layer.with_not_null(|layer| {
            image_cache.with_not_null(|image_cache| {
                shadow_cache.with_not_null(|shadow_cache| {
                    if let Some(MetalSurface {
                        mut surface,
                        drawable,
                        ..
                    }) = metal_context.new_surface()
                    {
                        SkiaCompositor::new(surface.canvas(), image_cache, shadow_cache)
                            .compose(layer.clone());

                        surface.flush_and_submit();
                        drop(surface);

                        let command_buffer = metal_context.queue.new_command_buffer();
                        command_buffer.present_drawable(drawable.as_ref());
                        command_buffer.commit();
                    }
                })
            })
        })
    })
}

#[no_mangle]
pub fn skia_metal_context_get_direct_context(
    metal_context: *mut ValueBox<MetalContext>,
) -> *mut ValueBox<DirectContext> {
    metal_context.with_not_null_return(std::ptr::null_mut(), |metal_context| {
        ValueBox::new(metal_context.direct_context.clone()).into_raw()
    })
}

#[no_mangle]
pub fn skia_metal_context_set_drawable_size(
    metal_context: *mut ValueBox<MetalContext>,
    width: f32,
    height: f32,
) {
    metal_context.with_not_null(|metal_context| {
        metal_context
            .layer
            .set_drawable_size(CGSize::new(width.into(), height.into()));
    })
}

#[no_mangle]
pub fn skia_metal_context_drop(ptr: &mut *mut ValueBox<MetalContext>) {
    ptr.drop();
}
