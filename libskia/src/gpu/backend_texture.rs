use skia_safe::gpu::{BackendAPI, BackendTexture};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[cfg(feature = "gl")]
#[no_mangle]
pub fn skia_backend_texture_new_gl(
    width: i32,
    height: i32,
    mip_mapped: bool,
    mut texture_info_ptr: BorrowedPtr<gpu::gl::TextureInfo>,
) -> OwnedPtr<BackendTexture> {
    texture_info_ptr
        .with_clone_ok(|texture_info| unsafe {
            OwnedPtr::new(gpu::backend_textures::make_gl(
                (width, height),
                if mip_mapped {
                    Mipmapped::Yes
                } else {
                    Mipmapped::No
                },
                texture_info,
                "Backend texture",
            ))
        })
        .or_log(OwnedPtr::null())
}

#[cfg(feature = "gl")]
#[no_mangle]
pub fn skia_backend_texture_get_gl_texture_info(
    mut texture_ptr: BorrowedPtr<BackendTexture>,
) -> OwnedPtr<gpu::gl::TextureInfo> {
    texture_ptr
        .with_mut_ok(|backend_texture| match backend_texture.gl_texture_info() {
            None => std::ptr::null_mut(),
            Some(texture_info) => OwnedPtr::new(texture_info),
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_backend_texture_get_width(texture_ptr: BorrowedPtr<BackendTexture>) -> i32 {
    texture_ptr
        .with_clone_ok(|backend_texture| backend_texture.width())
        .or_log(0)
}

#[no_mangle]
pub fn skia_backend_texture_get_height(texture_ptr: BorrowedPtr<BackendTexture>) -> i32 {
    texture_ptr
        .with_clone_ok(|backend_texture| backend_texture.height())
        .or_log(0)
}

#[no_mangle]
pub fn skia_backend_texture_has_mip_maps(texture_ptr: BorrowedPtr<BackendTexture>) -> bool {
    texture_ptr
        .with_clone_ok(|backend_texture| backend_texture.has_mipmaps())
        .or_log(false)
}

#[no_mangle]
pub fn skia_backend_texture_get_backend(texture_ptr: BorrowedPtr<BackendTexture>) -> BackendAPI {
    texture_ptr
        .with_clone_ok(|backend_texture| backend_texture.backend())
        .or_log(BackendAPI::Mock)
}

#[no_mangle]
pub fn skia_backend_texture_is_protected(texture_ptr: BorrowedPtr<BackendTexture>) -> bool {
    texture_ptr
        .with_clone_ok(|backend_texture| backend_texture.is_protected())
        .or_log(false)
}

#[no_mangle]
pub fn skia_backend_texture_is_valid(texture_ptr: BorrowedPtr<BackendTexture>) -> bool {
    texture_ptr
        .with_clone_ok(|backend_texture| backend_texture.is_valid())
        .or_log(false)
}

#[no_mangle]
pub fn skia_backend_texture_drop(ptr: OwnedPtr<BackendTexture>) {
    drop(ptr);
}
