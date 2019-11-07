use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{Image, ImageInfo, AlphaType, ColorType, ColorSpace};

#[no_mangle]
pub fn skia_image_get_image_info(_image_ptr: *mut ValueBox<Image>) -> *mut ValueBox<ImageInfo> {
    _image_ptr.with(|image| ValueBox::new(image.image_info().clone()).into_raw())
}

#[no_mangle]
pub fn skia_image_get_unique_id(_image_ptr: *mut ValueBox<Image>) -> u32 {
    _image_ptr.with(|image| image.unique_id())
}

#[no_mangle]
pub fn skia_image_get_alpha_type(_image_ptr: *mut ValueBox<Image>) -> AlphaType {
    _image_ptr.with(|image| image.alpha_type())
}

#[no_mangle]
pub fn skia_image_get_color_type(_image_ptr: *mut ValueBox<Image>) -> ColorType {
    _image_ptr.with(|image| image.color_type())
}

#[no_mangle]
pub fn skia_image_get_color_space(_image_ptr: *mut ValueBox<Image>) -> *mut ValueBox<ColorSpace> {
    _image_ptr.with(|image| ValueBox::new(image.color_space()).into_raw())
}

#[no_mangle]
pub fn skia_image_is_alpha_only(_image_ptr: *mut ValueBox<Image>) -> bool {
    _image_ptr.with(|image| image.is_alpha_only())
}

#[no_mangle]
pub fn skia_image_is_opaque(_image_ptr: *mut ValueBox<Image>) -> bool {
    _image_ptr.with(|image| image.is_opaque())
}

#[no_mangle]
pub fn skia_image_is_texture_backed(_image_ptr: *mut ValueBox<Image>) -> bool {
    _image_ptr.with(|image| image.is_texture_backed())
}

#[no_mangle]
pub fn skia_image_drop(_ptr: *mut ValueBox<Image>) {
    _ptr.drop()
}