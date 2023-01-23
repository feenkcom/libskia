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
pub fn skia_color_get_red(color: *mut ValueBox<Color>) -> u8 {
    color.with_clone_ok(Color::r).or_log(0)
}

#[no_mangle]
pub fn skia_color_get_green(color: *mut ValueBox<Color>) -> u8 {
    color.with_clone_ok(Color::g).or_log(0)
}

#[no_mangle]
pub fn skia_color_get_blue(color: *mut ValueBox<Color>) -> u8 {
    color.with_clone_ok(Color::b).or_log(0)
}

#[no_mangle]
pub fn skia_color_get_alpha(color: *mut ValueBox<Color>) -> u8 {
    color.with_clone_ok(Color::a).or_log(0)
}

#[no_mangle]
pub fn skia_color_drop(color: *mut ValueBox<Color>) {
    color.release();
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
        .with_clone_ok(|color| ArrayBox::from_vector(vec![color; amount]))
        .into_raw()
}

#[no_mangle]
pub fn skia_color_array_get_length(array: *mut ValueBox<ArrayBox<Color>>) -> usize {
    array.with_ref_ok(|array| array.length).or_log(0)
}

#[no_mangle]
pub fn skia_color_array_get_capacity(array: *mut ValueBox<ArrayBox<Color>>) -> usize {
    array.with_ref_ok(|array| array.capacity).or_log(0)
}

#[no_mangle]
pub fn skia_color_array_get_data(array: *mut ValueBox<ArrayBox<Color>>) -> *mut Color {
    array
        .with_ref_ok(|array| array.data)
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn skia_color_array_at(
    array: *mut ValueBox<ArrayBox<Color>>,
    index: usize,
) -> *mut ValueBox<Color> {
    array.with_ref_ok(|array| array.at(index)).into_raw()
}

#[no_mangle]
pub fn skia_color_array_at_put(
    array: *mut ValueBox<ArrayBox<Color>>,
    index: usize,
    color: *mut ValueBox<Color>,
) {
    color
        .with_ref(|color| array.with_mut_ok(|array| array.at_put(index, color.clone())))
        .log();
}

#[no_mangle]
pub fn skia_color_array_drop(ptr: *mut ValueBox<ArrayBox<Color>>) {
    ptr.release();
}

#[test]
pub fn test_skia_color_array() {
    let mut color = skia_color_default();
    let mut array_ptr = skia_color_array_create_with(color, 5);

    assert_eq!(color.has_value(), true);
    assert_eq!(array_ptr.has_value(), true);

    skia_color_array_drop(array_ptr);

    assert_eq!(color.has_value(), true);
    assert_eq!(array_ptr.has_value(), false);

    skia_color_drop(color);
    assert_eq!(color.has_value(), false);
}
