use skia_safe::gpu::{BackendAPI, BackendTexture};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[cfg(feature = "gl")]
#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_new_gl(
    width: i32,
    height: i32,
    mip_mapped: bool,
    texture_info: BorrowedPtr<skia_safe::gpu::gl::TextureInfo>,
) -> OwnedPtr<BackendTexture> {
    texture_info
        .with_clone_ok(|texture_info| unsafe {
            OwnedPtr::new(skia_safe::gpu::backend_textures::make_gl(
                (width, height),
                if mip_mapped {
                    skia_safe::gpu::Mipmapped::Yes
                } else {
                    skia_safe::gpu::Mipmapped::No
                },
                texture_info,
                "Backend texture",
            ))
        })
        .or_log(OwnedPtr::null())
}

#[cfg(feature = "gl")]
#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_get_gl_texture_info(
    mut texture: BorrowedPtr<BackendTexture>,
) -> OwnedPtr<skia_safe::gpu::gl::TextureInfo> {
    texture
        .with_mut_ok(|backend_texture| match backend_texture.gl_texture_info() {
            None => OwnedPtr::null(),
            Some(texture_info) => OwnedPtr::new(texture_info),
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_get_width(texture: BorrowedPtr<BackendTexture>) -> i32 {
    texture
        .with_clone_ok(|backend_texture| backend_texture.width())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_get_height(texture: BorrowedPtr<BackendTexture>) -> i32 {
    texture
        .with_clone_ok(|backend_texture| backend_texture.height())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_has_mip_maps(texture: BorrowedPtr<BackendTexture>) -> bool {
    texture
        .with_clone_ok(|backend_texture| backend_texture.has_mipmaps())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_get_backend(
    texture: BorrowedPtr<BackendTexture>,
) -> BackendAPI {
    texture
        .with_clone_ok(|backend_texture| backend_texture.backend())
        .or_log(BackendAPI::Mock)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_is_protected(texture: BorrowedPtr<BackendTexture>) -> bool {
    texture
        .with_clone_ok(|backend_texture| backend_texture.is_protected())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_is_valid(texture: BorrowedPtr<BackendTexture>) -> bool {
    texture.with_clone_ok(|_| true).or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_texture_drop(backend_texture: OwnedPtr<BackendTexture>) {
    drop(backend_texture);
}
