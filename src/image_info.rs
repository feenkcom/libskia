use skia_safe::{AlphaType, ColorType, ImageInfo, ISize, ColorSpace};
use boxer::CBox;

#[no_mangle]
pub fn skia_image_info_new_default() -> *mut ImageInfo {
    skia_image_info_new_s32(50, 50, AlphaType::Unpremul)
}

#[no_mangle]
pub fn skia_image_info_new(width: i32, height: i32, color_type: ColorType, alpha_type: AlphaType, _color_space_ptr: *mut ColorSpace) -> *mut ImageInfo {
    CBox::with_optional_raw(_color_space_ptr, |color_space_option| {
        CBox::into_raw(match color_space_option {
            None => { ImageInfo::new(ISize::new(width, height),color_type, alpha_type, None) },
            Some(color_space) => { ImageInfo::new(ISize::new(width, height),color_type, alpha_type, *(color_space.clone())) },
        })
    })
}

#[no_mangle]
pub fn skia_image_info_new_s32(width: i32, height: i32, alpha_type: AlphaType) -> *mut ImageInfo {
    CBox::into_raw(ImageInfo::new_s32(ISize::new(width, height),alpha_type))
}

#[no_mangle]
pub fn skia_image_info_min_row_bytes(_image_info_ptr: *mut ImageInfo) -> usize {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.min_row_bytes())
}

#[no_mangle]
pub fn skia_image_info_compute_byte_size(_image_info_ptr: *mut ImageInfo, row_bytes: usize) -> usize {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.compute_byte_size(row_bytes))
}

#[no_mangle]
pub fn skia_image_info_get_width(_image_info_ptr: *mut ImageInfo) -> i32 {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.width())
}

#[no_mangle]
pub fn skia_image_info_get_height(_image_info_ptr: *mut ImageInfo) -> i32 {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.height())
}

#[no_mangle]
pub fn skia_image_info_get_color_type(_image_info_ptr: *mut ImageInfo) -> ColorType {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.color_type())
}

#[no_mangle]
pub fn skia_image_info_get_alpha_type(_image_info_ptr: *mut ImageInfo) -> AlphaType {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.alpha_type())
}

#[no_mangle]
pub fn skia_image_info_get_bytes_per_pixel(_image_info_ptr: *mut ImageInfo) -> usize {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.bytes_per_pixel())
}

#[no_mangle]
pub fn skia_image_info_get_shift_per_pixel(_image_info_ptr: *mut ImageInfo) -> usize {
    CBox::with_raw(_image_info_ptr, |image_info| image_info.shift_per_pixel())
}

#[no_mangle]
pub fn skia_image_info_drop(_ptr: *mut ImageInfo) {
    CBox::drop(_ptr);
}