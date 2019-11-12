use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::image_filters::blur;
use skia_safe::{scalar, ImageFilter, TileMode};

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
pub fn skia_image_filter_drop(_ptr: *mut ValueBox<ImageFilter>) {
    _ptr.drop()
}
