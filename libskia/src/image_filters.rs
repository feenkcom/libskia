use skia_safe::image_filters::{CropRect, blur, drop_shadow, drop_shadow_only, image};
use skia_safe::{Color, Image, ImageFilter, Rect, SamplingOptions, TileMode, Vector, scalar};
use value_box::{BorrowedPtr, OwnedPtr};

#[unsafe(no_mangle)]
pub extern "C" fn skia_image_filter_blur(
    sigma_x: scalar,
    sigma_y: scalar,
    tile_mode: TileMode,
    input_ptr: BorrowedPtr<ImageFilter>,
) -> OwnedPtr<ImageFilter> {
    let filter_option = input_ptr
        .with_option_ref(|value| {
            Ok(match value {
                Some(input) => blur(
                    (sigma_x, sigma_y),
                    Some(tile_mode),
                    Some(input.clone()),
                    None,
                ),
                None => blur((sigma_x, sigma_y), Some(tile_mode), None, None),
            })
        })
        .expect("with_option_ref does not fail");

    match filter_option {
        None => OwnedPtr::null(),
        Some(filter) => OwnedPtr::new(filter),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_image_filter_image(
    image_ptr: BorrowedPtr<Image>,
    src_left: scalar,
    src_top: scalar,
    src_right: scalar,
    src_bottom: scalar,
    dst_left: scalar,
    dst_top: scalar,
    dst_right: scalar,
    dst_bottom: scalar,
) -> OwnedPtr<ImageFilter> {
    let filter_option = image_ptr
        .with_clone_ok(|image_source| {
            image(
                image_source,
                Rect::new(src_left, src_top, src_right, src_bottom).as_ref(),
                Rect::new(dst_left, dst_top, dst_right, dst_bottom).as_ref(),
                SamplingOptions::default(),
            )
        })
        .unwrap_or(None);
    match filter_option {
        None => OwnedPtr::null(),
        Some(filter) => OwnedPtr::new(filter),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_image_filter_drop_shadow(
    delta_x: scalar,
    delta_y: scalar,
    sigma_x: scalar,
    sigma_y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    input_ptr: BorrowedPtr<ImageFilter>,
) -> OwnedPtr<ImageFilter> {
    let filter_option = input_ptr
        .with_option_ref(|value| {
            Ok(match value {
                Some(input) => drop_shadow(
                    Vector::new(delta_x, delta_y),
                    (sigma_x, sigma_y),
                    Color::from_argb(a, r, g, b),
                    None,
                    Some(input.clone()),
                    CropRect::NO_CROP_RECT,
                ),
                None => drop_shadow(
                    Vector::new(delta_x, delta_y),
                    (sigma_x, sigma_y),
                    Color::from_argb(a, r, g, b),
                    None,
                    None,
                    CropRect::NO_CROP_RECT,
                ),
            })
        })
        .expect("with_option_ref does not fail");

    match filter_option {
        None => OwnedPtr::null(),
        Some(filter) => OwnedPtr::new(filter),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_image_filter_drop_shadow_only(
    delta_x: scalar,
    delta_y: scalar,
    sigma_x: scalar,
    sigma_y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    input_ptr: BorrowedPtr<ImageFilter>,
) -> OwnedPtr<ImageFilter> {
    let filter_option = input_ptr
        .with_option_ref(|value| {
            Ok(match value {
                Some(input) => drop_shadow_only(
                    Vector::new(delta_x, delta_y),
                    (sigma_x, sigma_y),
                    Color::from_argb(a, r, g, b),
                    None,
                    Some(input.clone()),
                    CropRect::NO_CROP_RECT,
                ),
                None => drop_shadow_only(
                    Vector::new(delta_x, delta_y),
                    (sigma_x, sigma_y),
                    Color::from_argb(a, r, g, b),
                    None,
                    None,
                    CropRect::NO_CROP_RECT,
                ),
            })
        })
        .expect("with_option_ref does not fail");
    match filter_option {
        None => OwnedPtr::null(),
        Some(filter) => OwnedPtr::new(filter),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_image_filter_drop(ptr: OwnedPtr<ImageFilter>) {
    drop(ptr);
}
