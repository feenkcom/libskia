use skia_safe::ImageFilter;
use boxer::boxes::ValueBox;

#[no_mangle]
pub fn skia_image_filters_blur() -> *mut ValueBox<ImageFilter> {
    std::ptr::null_mut()
}