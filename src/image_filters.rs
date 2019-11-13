use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::image_filters::{blur, image};
use skia_safe::{scalar, ImageFilter, TileMode, Image, FilterQuality, Rect};

#[no_mangle]
pub fn skia_image_filter_blur(
    sigma_x: scalar,
    sigma_y: scalar,
    tile_mode: TileMode,
    input_ptr: *mut ValueBox<ImageFilter>,
) -> *mut ValueBox<ImageFilter> {
    let filter_option = match input_ptr.as_option() {
        None => blur((sigma_x, sigma_y), Some(tile_mode), None, None),
        Some(mut _input_ptr) => {
            _input_ptr.with_value_consumed(|input| blur((sigma_x, sigma_y), Some(tile_mode), Some(input), None))
        }
    };
    match filter_option {
        None => std::ptr::null_mut(),
        Some(filter) => ValueBox::new(filter).into_raw(),
    }
}

#[no_mangle]
pub fn skia_image_filter_image(
    mut image_ptr: *mut ValueBox<Image>,
    src_left: scalar, src_top: scalar, src_right: scalar, src_bottom: scalar,
    dst_left: scalar, dst_top: scalar, dst_right: scalar, dst_bottom: scalar,
    filter_quality: FilterQuality,
) -> *mut ValueBox<ImageFilter> {
    let filter_option = image_ptr.with_value_consumed(|image_source|{
        image(image_source,Rect::new(src_left, src_top, src_right, src_bottom).as_ref(), Rect::new(dst_left, dst_top, dst_right, dst_bottom).as_ref(), filter_quality)
    });
    match filter_option {
        None => std::ptr::null_mut(),
        Some(filter) => ValueBox::new(filter).into_raw(),
    }
}

#[no_mangle]
pub fn skia_image_filter_drop(_ptr: *mut ValueBox<ImageFilter>) {
    _ptr.drop()
}
