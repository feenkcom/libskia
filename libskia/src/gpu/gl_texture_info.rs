use crate::value_box_compat::*;
use skia_safe::gpu::gl::TextureInfo;
use value_box::{BorrowedPtr, OwnedPtr};

#[no_mangle]
pub fn skia_texture_info_default() -> OwnedPtr<TextureInfo> {
    OwnedPtr::new(TextureInfo::default()).into_raw()
}

#[no_mangle]
pub fn skia_texture_info_from_target_and_id(
    target: std::os::raw::c_uint,
    id: std::os::raw::c_uint,
) -> OwnedPtr<TextureInfo> {
    OwnedPtr::new(TextureInfo::from_target_and_id(target, id)).into_raw()
}

#[no_mangle]
pub fn skia_texture_info_get_target(
    mut texture_info: BorrowedPtr<TextureInfo>,
) -> std::os::raw::c_uint {
    texture_info.with_not_null_return(0, |texture_info| texture_info.target)
}

#[no_mangle]
pub fn skia_texture_info_get_format(
    mut texture_info: BorrowedPtr<TextureInfo>,
) -> std::os::raw::c_uint {
    texture_info.with_not_null_return(0, |texture_info| texture_info.format)
}

#[no_mangle]
pub fn skia_texture_info_set_format(
    mut _ptr: BorrowedPtr<TextureInfo>,
    format: std::os::raw::c_uint,
) {
    _ptr.with_not_null(|texture_info| texture_info.format = format);
}

#[no_mangle]
pub fn skia_texture_info_get_id(
    mut texture_info_ptr: BorrowedPtr<TextureInfo>,
) -> std::os::raw::c_uint {
    texture_info_ptr.with_not_null_return(0, |texture_info| texture_info.id)
}

#[no_mangle]
pub fn skia_texture_info_drop(mut ptr: OwnedPtr<TextureInfo>) {
    ptr.release();
}
