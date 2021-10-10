use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::image_filters::{blur, drop_shadow, drop_shadow_only, image};
use skia_safe::{
    scalar, Color, FilterQuality, Image, ImageFilter, Rect, SamplingOptions, TileMode, Vector,
};

#[no_mangle]
pub fn skia_image_filter_blur(
    sigma_x: scalar,
    sigma_y: scalar,
    tile_mode: TileMode,
    input_ptr: *mut ValueBox<ImageFilter>,
) -> *mut ValueBox<ImageFilter> {
    let filter_option = input_ptr.with_value(
        || blur((sigma_x, sigma_y), Some(tile_mode), None, None),
        |input| blur((sigma_x, sigma_y), Some(tile_mode), Some(input), None),
    );

    match filter_option {
        None => std::ptr::null_mut(),
        Some(filter) => ValueBox::new(filter).into_raw(),
    }
}

#[no_mangle]
pub fn skia_image_filter_image(
    image_ptr: *mut ValueBox<Image>,
    src_left: scalar,
    src_top: scalar,
    src_right: scalar,
    src_bottom: scalar,
    dst_left: scalar,
    dst_top: scalar,
    dst_right: scalar,
    dst_bottom: scalar,
    filter_quality: FilterQuality,
) -> *mut ValueBox<ImageFilter> {
    let filter_option = image_ptr.with_not_null_value_return(None, |image_source| {
        image(
            image_source,
            Rect::new(src_left, src_top, src_right, src_bottom).as_ref(),
            Rect::new(dst_left, dst_top, dst_right, dst_bottom).as_ref(),
            SamplingOptions::from(filter_quality),
        )
    });
    match filter_option {
        None => std::ptr::null_mut(),
        Some(filter) => ValueBox::new(filter).into_raw(),
    }
}

#[no_mangle]
pub fn skia_image_filter_drop_shadow(
    delta_x: scalar,
    delta_y: scalar,
    sigma_x: scalar,
    sigma_y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    input_ptr: *mut ValueBox<ImageFilter>,
) -> *mut ValueBox<ImageFilter> {
    let filter_option = input_ptr.with_value(
        || {
            drop_shadow(
                Vector::new(delta_x, delta_y),
                (sigma_x, sigma_y),
                Color::from_argb(a, r, g, b),
                None,
                None,
            )
        },
        |input| {
            drop_shadow(
                Vector::new(delta_x, delta_y),
                (sigma_x, sigma_y),
                Color::from_argb(a, r, g, b),
                Some(input),
                None,
            )
        },
    );

    match filter_option {
        None => std::ptr::null_mut(),
        Some(filter) => ValueBox::new(filter).into_raw(),
    }
}

#[no_mangle]
pub fn skia_image_filter_drop_shadow_only(
    delta_x: scalar,
    delta_y: scalar,
    sigma_x: scalar,
    sigma_y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    input_ptr: *mut ValueBox<ImageFilter>,
) -> *mut ValueBox<ImageFilter> {
    let filter_option = input_ptr.with_value(
        || {
            drop_shadow_only(
                Vector::new(delta_x, delta_y),
                (sigma_x, sigma_y),
                Color::from_argb(a, r, g, b),
                None,
                None,
            )
        },
        |input| {
            drop_shadow_only(
                Vector::new(delta_x, delta_y),
                (sigma_x, sigma_y),
                Color::from_argb(a, r, g, b),
                Some(input),
                None,
            )
        },
    );
    match filter_option {
        None => std::ptr::null_mut(),
        Some(filter) => ValueBox::new(filter).into_raw(),
    }
}

#[no_mangle]
pub fn skia_image_filter_drop(ptr: &mut *mut ValueBox<ImageFilter>) {
    drop!(ptr);
}
