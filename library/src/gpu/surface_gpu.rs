use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::{BackendRenderTarget, BackendTexture, DirectContext, SurfaceOrigin};
use skia_safe::{Budgeted, ColorType, ImageInfo, Surface};

#[no_mangle]
pub fn skia_surface_from_render_target(
    backend_render_target_ptr: *mut ValueBox<BackendRenderTarget>,
    context_ptr: *mut ValueBox<DirectContext>,
    color_type: ColorType,
) -> *mut ValueBox<Surface> {
    backend_render_target_ptr.with_not_null_return(std::ptr::null_mut(),|backend_render_target| {
        context_ptr.with_not_null_return(std::ptr::null_mut(), |context| {
            let surface_option = Surface::from_backend_render_target(
            context,
            backend_render_target,
            SurfaceOrigin::BottomLeft,
            color_type,
            None,
            None
        );
        match surface_option {
            None => {
                error!("[skia_surface_from_render_target] Unable to create Skia Surface width: {:?} height: {:?} color type: {:?}",
                    backend_render_target.width(),
                    backend_render_target.height(),
                    color_type);
                std::ptr::null_mut() },
            Some(surface) => {
                ValueBox::new(surface).into_raw()
            },
        }
        })
    })
}

#[no_mangle]
pub fn skia_surface_new_render_target(
    image_info: *mut ValueBox<ImageInfo>,
    direct_context: *mut ValueBox<DirectContext>,
) -> *mut ValueBox<Surface> {
    image_info.with_not_null_return(std::ptr::null_mut(), |image_info| {
        direct_context.with_not_null_return(std::ptr::null_mut(), |direct_context| {
            let surface_option = Surface::new_render_target(
                direct_context,
                Budgeted::No,
                &image_info,
                0,
                SurfaceOrigin::BottomLeft,
                None,
                true,
        );
        match surface_option {
            None => {
                error!("[skia_surface_new_render_target] Unable to create Skia Surface width: {:?} height: {:?} color type: {:?}",
                    image_info.width(),
                    image_info.height(),
                    image_info.color_type());
                std::ptr::null_mut() },
            Some(surface) => {
                ValueBox::new(surface).into_raw()
            },
        }
        })

    })
}

#[no_mangle]
pub fn skia_surface_from_backend_texture(
    context_ptr: *mut ValueBox<DirectContext>,
    backend_texture_ptr: *mut ValueBox<BackendTexture>,
    color_type: ColorType,
) -> *mut ValueBox<Surface> {
    backend_texture_ptr.with_not_null_return(std::ptr::null_mut(),|backend_texture| {
        context_ptr.with_not_null_return(std::ptr::null_mut(), |context| {
            let surface_option = Surface::from_backend_texture(
            context,
            backend_texture,
            SurfaceOrigin::BottomLeft,
            None,
            color_type,
            None,
            None
        );
        match surface_option {
            None => {
                error!("[skia_surface_from_backend_texture] Unable to create Skia Surface from backend texture width: {:?} height: {:?} color type: {:?}",
                    backend_texture.width(),
                    backend_texture.height(),
                    color_type);
                std::ptr::null_mut() },
            Some(surface) => {
                ValueBox::new(surface).into_raw()
            },
        }
        })
    })
}
