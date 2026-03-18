use skia_safe::gpu::gl::TextureInfo;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_texture_info_default() -> OwnedPtr<TextureInfo> {
    OwnedPtr::new(TextureInfo::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_texture_info_from_target_and_id(
    target: std::os::raw::c_uint,
    id: std::os::raw::c_uint,
) -> OwnedPtr<TextureInfo> {
    OwnedPtr::new(TextureInfo::from_target_and_id(target, id))
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_texture_info_get_target(
    mut texture_info: BorrowedPtr<TextureInfo>,
) -> std::os::raw::c_uint {
    texture_info
        .with_mut_ok(|texture_info| texture_info.target)
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_texture_info_get_format(
    mut texture_info: BorrowedPtr<TextureInfo>,
) -> std::os::raw::c_uint {
    texture_info
        .with_mut_ok(|texture_info| texture_info.format)
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_texture_info_set_format(
    mut texture_info: BorrowedPtr<TextureInfo>,
    format: std::os::raw::c_uint,
) {
    texture_info
        .with_mut_ok(|texture_info| texture_info.format = format)
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_texture_info_get_id(
    mut texture_info: BorrowedPtr<TextureInfo>,
) -> std::os::raw::c_uint {
    texture_info
        .with_mut_ok(|texture_info| texture_info.id)
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_texture_info_drop(mut texture_info: OwnedPtr<TextureInfo>) {
    drop(texture_info);
}
