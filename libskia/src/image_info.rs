use skia_safe::{AlphaType, ColorSpace, ColorType, ISize, ImageInfo};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_image_info_new_default() -> OwnedPtr<ImageInfo> {
    OwnedPtr::new(ImageInfo::default())
}

#[no_mangle]
pub fn skia_image_info_new(
    width: i32,
    height: i32,
    color_type: ColorType,
    alpha_type: AlphaType,
) -> OwnedPtr<ImageInfo> {
    OwnedPtr::new(ImageInfo::new(
        ISize::new(width, height),
        color_type,
        alpha_type,
        None,
    ))
}

#[no_mangle]
pub fn skia_image_info_new_with_color_space(
    width: i32,
    height: i32,
    color_type: ColorType,
    alpha_type: AlphaType,
    color_space_ptr: BorrowedPtr<ColorSpace>,
) -> OwnedPtr<ImageInfo> {
    color_space_ptr
        .with_clone_ok(|color_space| {
            OwnedPtr::new(ImageInfo::new(
                ISize::new(width, height),
                color_type,
                alpha_type,
                color_space,
            ))
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_image_info_new_s32(
    width: i32,
    height: i32,
    alpha_type: AlphaType,
) -> OwnedPtr<ImageInfo> {
    OwnedPtr::new(ImageInfo::new_s32(ISize::new(width, height), alpha_type))
}

#[no_mangle]
pub fn skia_image_info_min_row_bytes(image_info_ptr: BorrowedPtr<ImageInfo>) -> usize {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.min_row_bytes())
        .or_log(0)
}

#[no_mangle]
pub fn skia_image_info_compute_byte_size(
    image_info_ptr: BorrowedPtr<ImageInfo>,
    row_bytes: usize,
) -> usize {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.compute_byte_size(row_bytes))
        .or_log(0)
}

#[no_mangle]
pub fn skia_image_info_get_width(image_info_ptr: BorrowedPtr<ImageInfo>) -> i32 {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.width())
        .or_log(0)
}

#[no_mangle]
pub fn skia_image_info_get_height(image_info_ptr: BorrowedPtr<ImageInfo>) -> i32 {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.height())
        .or_log(0)
}

#[no_mangle]
pub fn skia_image_info_get_color_type(image_info_ptr: BorrowedPtr<ImageInfo>) -> ColorType {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.color_type())
        .or_log(ColorType::Unknown)
}

#[no_mangle]
pub fn skia_image_info_get_alpha_type(image_info_ptr: BorrowedPtr<ImageInfo>) -> AlphaType {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.alpha_type())
        .or_log(AlphaType::Unknown)
}

#[no_mangle]
pub fn skia_image_info_get_bytes_per_pixel(image_info_ptr: BorrowedPtr<ImageInfo>) -> usize {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.bytes_per_pixel())
        .or_log(0)
}

#[no_mangle]
pub fn skia_image_info_get_shift_per_pixel(image_info_ptr: BorrowedPtr<ImageInfo>) -> usize {
    image_info_ptr
        .with_clone_ok(|image_info| image_info.shift_per_pixel())
        .or_log(0)
}

#[no_mangle]
pub fn skia_image_info_drop(ptr: OwnedPtr<ImageInfo>) {
    drop(ptr);
}
