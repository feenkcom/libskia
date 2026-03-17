use crate::value_box_compat::*;
use skia_safe::gpu::Budgeted;
use skia_safe::gpu::{BackendRenderTarget, BackendTexture, DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, ImageInfo, Surface};
use value_box::{BorrowedPtr, OwnedPtr};

#[no_mangle]
pub fn skia_surface_from_render_target(
    backend_render_target_ptr: BorrowedPtr<BackendRenderTarget>,
    context_ptr: BorrowedPtr<DirectContext>,
    color_type: ColorType,
) -> OwnedPtr<Surface> {
    backend_render_target_ptr.with_not_null_return(OwnedPtr::null(), |backend_render_target| {
        context_ptr.with_not_null_return(OwnedPtr::null(), |context| {
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
                OwnedPtr::null() },
            Some(surface) => {
                OwnedPtr::new(surface)
            },
        }
        })
    })
}

#[no_mangle]
pub fn skia_surface_new_render_target(
    image_info: BorrowedPtr<ImageInfo>,
    direct_context: BorrowedPtr<DirectContext>,
) -> OwnedPtr<Surface> {
    image_info.with_not_null_return(OwnedPtr::null(), |image_info| {
        direct_context.with_not_null_return(OwnedPtr::null(), |direct_context| {
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
                OwnedPtr::null() },
            Some(surface) => {
                OwnedPtr::new(surface)
            },
        }
        })

    })
}

#[no_mangle]
pub fn skia_surface_from_backend_texture(
    context_ptr: BorrowedPtr<DirectContext>,
    backend_texture_ptr: BorrowedPtr<BackendTexture>,
    color_type: ColorType,
) -> OwnedPtr<Surface> {
    backend_texture_ptr.with_not_null_return(OwnedPtr::null(), |backend_texture| {
        context_ptr.with_not_null_return(OwnedPtr::null(), |context| {
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
                OwnedPtr::null() },
            Some(surface) => {
                OwnedPtr::new(surface)
            },
        }
        })
    })
}
