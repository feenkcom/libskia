use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::gl::TextureInfo;
use skia_safe::gpu::{BackendTexture, MipMapped, BackendAPI};

#[no_mangle]
pub fn skia_backend_texture_new_gl(
    width: i32,
    height: i32,
    mip_mapped: bool,
    texture_info_ptr: *mut ValueBox<TextureInfo>,
) -> *mut ValueBox<BackendTexture> {
    texture_info_ptr.with_value(|texture_info| {
        ValueBox::new(unsafe {
            BackendTexture::new_gl(
                (width, height),
                if mip_mapped {
                    MipMapped::Yes
                } else {
                    MipMapped::No
                },
                texture_info,
            )
        })
        .into_raw()
    })
}

#[no_mangle]
pub fn skia_backend_texture_get_width(_ptr: *mut ValueBox<BackendTexture>) -> i32 {
    _ptr.with_not_null_return(0, |backend_texture| backend_texture.width())
}

#[no_mangle]
pub fn skia_backend_texture_get_height(_ptr: *mut ValueBox<BackendTexture>) -> i32 {
    _ptr.with_not_null_return(0, |backend_texture| backend_texture.height())
}

#[no_mangle]
pub fn skia_backend_texture_has_mip_maps(_ptr: *mut ValueBox<BackendTexture>) -> bool {
    _ptr.with_not_null_return(false, |backend_texture| backend_texture.has_mip_maps())
}

#[no_mangle]
pub fn skia_backend_texture_get_backend(_ptr: *mut ValueBox<BackendTexture>) -> BackendAPI {
    _ptr.with_not_null_return(BackendAPI::Mock, |backend_texture| backend_texture.backend())
}

#[no_mangle]
pub fn skia_backend_texture_is_protected(_ptr: *mut ValueBox<BackendTexture>) -> bool {
    _ptr.with_not_null_return(false, |backend_texture| backend_texture.is_protected())
}

#[no_mangle]
pub fn skia_backend_texture_is_valid(_ptr: *mut ValueBox<BackendTexture>) -> bool {
    _ptr.with_not_null_return(false, |backend_texture| backend_texture.is_valid())
}

#[no_mangle]
pub fn skia_backend_texture_get_gl_texture_info(_ptr: *mut ValueBox<BackendTexture>) -> *mut ValueBox<TextureInfo> {
    _ptr.with_not_null_return(std::ptr::null_mut(), |backend_texture| match backend_texture.gl_texture_info() {
        None => { std::ptr::null_mut() },
        Some(texture_info) => { ValueBox::new(texture_info).into_raw()},
    })
}

#[no_mangle]
pub fn skia_backend_texture_drop(_ptr: *mut ValueBox<BackendTexture>) {
    _ptr.drop();
}