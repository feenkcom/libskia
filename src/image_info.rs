use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::{AlphaType, ColorSpace, ColorType, ISize, ImageInfo};

#[no_mangle]
pub fn skia_image_info_new_default() -> *mut ValueBox<ImageInfo> {
    ValueBox::new(ImageInfo::default()).into_raw()
}

#[no_mangle]
pub fn skia_image_info_new(
    width: i32,
    height: i32,
    color_type: ColorType,
    alpha_type: AlphaType,
) -> *mut ValueBox<ImageInfo> {
    ValueBox::new(ImageInfo::new(
        ISize::new(width, height),
        color_type,
        alpha_type,
        None,
    ))
    .into_raw()
}

#[no_mangle]
pub fn skia_image_info_new_with_color_space(
    width: i32,
    height: i32,
    color_type: ColorType,
    alpha_type: AlphaType,
    color_space_ptr: *mut ValueBox<ColorSpace>,
) -> *mut ValueBox<ImageInfo> {
    color_space_ptr.with_not_null_value_return(std::ptr::null_mut(), |color_space| {
        ValueBox::new(ImageInfo::new(
            ISize::new(width, height),
            color_type,
            alpha_type,
            color_space,
        ))
        .into_raw()
    })
}

#[no_mangle]
pub fn skia_image_info_new_s32(
    width: i32,
    height: i32,
    alpha_type: AlphaType,
) -> *mut ValueBox<ImageInfo> {
    ValueBox::new(ImageInfo::new_s32(ISize::new(width, height), alpha_type)).into_raw()
}

#[no_mangle]
pub fn skia_image_info_min_row_bytes(image_info_ptr: *mut ValueBox<ImageInfo>) -> usize {
    image_info_ptr.with_not_null_return(0, |image_info| image_info.min_row_bytes())
}

#[no_mangle]
pub fn skia_image_info_compute_byte_size(
    image_info_ptr: *mut ValueBox<ImageInfo>,
    row_bytes: usize,
) -> usize {
    image_info_ptr.with_not_null_return(0, |image_info| image_info.compute_byte_size(row_bytes))
}

#[no_mangle]
pub fn skia_image_info_get_width(image_info_ptr: *mut ValueBox<ImageInfo>) -> i32 {
    image_info_ptr.with_not_null_return(0, |image_info| image_info.width())
}

#[no_mangle]
pub fn skia_image_info_get_height(image_info_ptr: *mut ValueBox<ImageInfo>) -> i32 {
    image_info_ptr.with_not_null_return(0, |image_info| image_info.height())
}

#[no_mangle]
pub fn skia_image_info_get_color_type(image_info_ptr: *mut ValueBox<ImageInfo>) -> ColorType {
    image_info_ptr.with_not_null_return(ColorType::Unknown, |image_info| image_info.color_type())
}

#[no_mangle]
pub fn skia_image_info_get_alpha_type(image_info_ptr: *mut ValueBox<ImageInfo>) -> AlphaType {
    image_info_ptr.with_not_null_return(AlphaType::Unknown, |image_info| image_info.alpha_type())
}

#[no_mangle]
pub fn skia_image_info_get_bytes_per_pixel(image_info_ptr: *mut ValueBox<ImageInfo>) -> usize {
    image_info_ptr.with_not_null_return(0, |image_info| image_info.bytes_per_pixel())
}

#[no_mangle]
pub fn skia_image_info_get_shift_per_pixel(image_info_ptr: *mut ValueBox<ImageInfo>) -> usize {
    image_info_ptr.with_not_null_return(0, |image_info| image_info.shift_per_pixel())
}

#[no_mangle]
pub fn skia_image_info_drop(mut ptr: *mut ValueBox<ImageInfo>) {
    ptr.drop();
}
