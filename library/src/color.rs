use std::ops::Deref;

use array_box::ArrayBox;
use skia_safe::Color;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_color_default() -> *mut ValueBox<Color> {
    ValueBox::new(Color::default()).into_raw()
}

#[no_mangle]
pub fn skia_color_create(r: u8, g: u8, b: u8, a: u8) -> *mut ValueBox<Color> {
    ValueBox::new(Color::from_argb(a, r, g, b)).into_raw()
}

#[no_mangle]
pub fn skia_color_create_argb(argb: u32) -> *mut ValueBox<Color> {
    ValueBox::new(Color::new(argb)).into_raw()
}

#[no_mangle]
pub fn skia_color_get_red(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.r())
}

#[no_mangle]
pub fn skia_color_get_green(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.g())
}

#[no_mangle]
pub fn skia_color_get_blue(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.b())
}

#[no_mangle]
pub fn skia_color_get_alpha(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.a())
}

#[no_mangle]
pub fn skia_color_drop(ptr: *mut ValueBox<Color>) {
    ptr.release();
}

#[no_mangle]
pub fn skia_color_array_default() -> *mut ValueBox<ArrayBox<Color>> {
    ValueBox::new(ArrayBox::new()).into_raw()
}

#[no_mangle]
pub fn skia_color_array_create_with(
    color: *mut ValueBox<Color>,
    amount: usize,
) -> *mut ValueBox<ArrayBox<Color>> {
    color
        .to_ref()
        .map(|color| ArrayBox::<Color>::from_vector(vec![color.deref().clone(); amount]))
        .into_raw()
}

#[no_mangle]
pub fn skia_color_array_get_length(array: *mut ValueBox<ArrayBox<Color>>) -> usize {
    array.to_ref().map(|array| array.length).or_log(0)
}

#[no_mangle]
pub fn skia_color_array_get_capacity(array: *mut ValueBox<ArrayBox<Color>>) -> usize {
    array.to_ref().map(|array| array.capacity).or_log(0)
}

#[no_mangle]
pub fn skia_color_array_get_data(array: *mut ValueBox<ArrayBox<Color>>) -> *mut Color {
    array
        .to_ref()
        .map(|array| array.data)
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn skia_color_array_at(
    array_ptr: *mut ValueBox<ArrayBox<Color>>,
    index: usize,
) -> *mut ValueBox<Color> {
    array_ptr.with_not_null_return(std::ptr::null_mut(), |array| {
        ValueBox::new(array.to_slice()[index]).into_raw()
    })
}

#[no_mangle]
pub fn skia_color_array_at_put(
    array: *mut ValueBox<ArrayBox<Color>>,
    index: usize,
    color: *mut ValueBox<Color>,
) {
    color
        .to_ref()
        .and_then(|color| {
            array
                .to_ref()
                .map(|mut array| array.at_put(index, color.clone()))
        })
        .log();
}

#[no_mangle]
pub fn skia_color_array_drop(ptr: *mut ValueBox<ArrayBox<Color>>) {
    ptr.release();
}

#[test]
pub fn test_skia_color_array() {
    let mut color_ptr = skia_color_default();
    let mut array_ptr = skia_color_array_create_with(color_ptr, 5);

    assert_eq!(color_ptr.has_value(), true);
    assert_eq!(array_ptr.has_value(), true);

    skia_color_array_drop(array_ptr);

    assert_eq!(color_ptr.has_value(), true);
    assert_eq!(array_ptr.has_value(), false);

    skia_color_drop(color_ptr);
    assert_eq!(color_ptr.has_value(), false);
}
