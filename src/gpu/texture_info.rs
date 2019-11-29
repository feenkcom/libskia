use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::gl::TextureInfo;

#[no_mangle]
pub fn skia_texture_info_default() -> *mut ValueBox<TextureInfo> {
    ValueBox::new(TextureInfo::default()).into_raw()
}

#[no_mangle]
pub fn skia_texture_info_from_target_and_id(target: std::os::raw::c_uint, id: std::os::raw::c_uint) -> *mut ValueBox<TextureInfo> {
    ValueBox::new(TextureInfo::from_target_and_id(target, id)).into_raw()
}

#[no_mangle]
pub fn skia_texture_info_get_target(_ptr: *mut ValueBox<TextureInfo>) -> std::os::raw::c_uint {
    _ptr.with_not_null_return(0, |texture_info| texture_info.target)
}

#[no_mangle]
pub fn skia_texture_info_get_format(_ptr: *mut ValueBox<TextureInfo>) -> std::os::raw::c_uint {
    _ptr.with_not_null_return(0, |texture_info| texture_info.format)
}

#[no_mangle]
pub fn skia_texture_info_set_format(_ptr: *mut ValueBox<TextureInfo>, format: std::os::raw::c_uint) {
    _ptr.with_not_null(|texture_info| texture_info.format = format);
}

#[no_mangle]
pub fn skia_texture_info_get_id(_ptr: *mut ValueBox<TextureInfo>) -> std::os::raw::c_uint {
    _ptr.with_not_null_return(0, |texture_info| texture_info.id)
}

#[no_mangle]
pub fn skia_texture_info_drop(_ptr: *mut ValueBox<TextureInfo>) {
    _ptr.drop();
}