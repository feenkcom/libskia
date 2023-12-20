use crate::gpu::platform_compositor::{PlatformCompositor, PlatformContext};

use cocoa::base::{id as cocoa_id, YES};

use uikit_sys::{
    id as uikit_id, CALayer, CGPoint as UIPoint, CGRect as UIRect, CGSize as UISize, ICALayer,
    INSObject, IUIColor, IUIView, NSObject, PCALayerDelegate, UIColor, UIResponder, UIView,
    UIView_UIViewHierarchy, UIView_UIViewRendering,
};

use objc::declare::ClassDecl;
use objc::runtime::Class;
use objc::runtime::Object;
use objc::runtime::Sel;
use objc::*;

use core_graphics_types::geometry::CGSize;
use foreign_types_shared::{ForeignType, ForeignTypeRef};
use metal::{CommandQueue, Device, MTLPixelFormat, MetalDrawableRef, MetalLayer};
use skia_safe::gpu::mtl::BackendContext;
use skia_safe::gpu::{mtl, BackendRenderTarget, DirectContext, SurfaceOrigin};
use skia_safe::{gpu, scalar, ColorType, ISize, Size, Surface};
use std::fmt::{Debug, Formatter};
use std::mem;
use std::mem::transmute;
use value_box::ValueBox;

#[allow(dead_code)]
pub struct MetalContext {
    device: Device,
    layer: MetalLayer,
    queue: CommandQueue,
    backend_context: BackendContext,
    direct_context: DirectContext,
    ui_view: UIView,
    metal_view: UIView,
}

impl MetalContext {
    pub fn new(ns_view: cocoa_id, size: Option<CGSize>) -> Self {
        let device = Device::system_default().expect("no device found");
        let ui_view = UIView(ns_view as uikit_id);

        create_metal_view_class();
        let metal_view = unsafe {
            let sub_view = unsafe {
                let cls = class!(MetalView);
                let obj: *mut Object = msg_send![cls, alloc];
                UIView(obj)
            };
            if let Some(size) = size {
                let frame = UIRect {
                    origin: UIPoint { x: 0.0, y: 0.0 },
                    size: UISize {
                        width: size.width,
                        height: size.height,
                    },
                };
                sub_view.initWithFrame_(frame);
            } else {
                sub_view.init();
            }
            sub_view
        };

        unsafe {
            ui_view.addSubview_(metal_view.clone());
        }

        let layer =
            {
                let layer: MetalLayer = unsafe { transmute(metal_view.clone().layer()) };
                layer.set_device(&device);
                layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
                layer.set_presents_with_transaction(true);
                if let Some(size) = size {
                    layer.set_drawable_size(size);
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
            ui_view,
            metal_view,
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
                &texture_info,
            );

            if let Some(mut surface) = gpu::surfaces::wrap_backend_render_target(
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

impl Debug for MetalContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MetalContext").finish()
    }
}

fn create_metal_view_class() {
    let superclass = class!(UIView);
    let mut decl = ClassDecl::new("MetalView", superclass).unwrap();

    // Add an ObjC method for getting the number
    extern "C" fn layerClass(this: &Class, _cmd: Sel) -> *const Class {
        class!(CAMetalLayer)
    }
    unsafe {
        decl.add_class_method(
            sel!(layerClass),
            layerClass as extern "C" fn(&Class, Sel) -> *const Class,
        );
    }

    decl.register();
}

#[no_mangle]
pub fn skia_metal_compositor_new_size(
    ns_view: cocoa_id,
    width: u32,
    height: u32,
) -> *mut ValueBox<PlatformCompositor> {
    ValueBox::new(PlatformCompositor::new(PlatformContext::Metal(
        MetalContext::new(ns_view, Some(CGSize::new(width.into(), height.into()))),
    )))
    .into_raw()
}
