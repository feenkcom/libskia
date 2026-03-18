use log::error;
use skia_safe::gpu::Budgeted;
use skia_safe::gpu::{BackendRenderTarget, BackendTexture, DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, ImageInfo, Surface, gpu};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_from_render_target(
    backend_render_target: BorrowedPtr<BackendRenderTarget>,
    mut context: BorrowedPtr<DirectContext>,
    color_type: ColorType,
) -> OwnedPtr<Surface> {
    backend_render_target.with_ref(|backend_render_target| {
        context.with_mut_ok(|context| {
            let surface_option = gpu::surfaces::wrap_backend_render_target(
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
    .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_new_render_target(
    image_info: BorrowedPtr<ImageInfo>,
    mut direct_context: BorrowedPtr<DirectContext>,
) -> OwnedPtr<Surface> {
    image_info.with_ref(|image_info| {
        direct_context.with_mut_ok(|direct_context| {
            let surface_option = gpu::surfaces::render_target(
                direct_context,
                Budgeted::No,
                image_info,
                0,
                SurfaceOrigin::BottomLeft,
                None,
                true,
                None
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
    .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_from_backend_texture(
    mut context: BorrowedPtr<DirectContext>,
    backend_texture: BorrowedPtr<BackendTexture>,
    color_type: ColorType,
) -> OwnedPtr<Surface> {
    backend_texture.with_ref(|backend_texture| {
        context.with_mut_ok(|context| {
            let surface_option = gpu::surfaces::wrap_backend_texture(
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
    .or_log(OwnedPtr::null())
}
