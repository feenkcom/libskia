use array_box::ArrayBox;
use skia_safe::Color;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_color_default() -> OwnedPtr<Color> {
    OwnedPtr::new(Color::default())
}

#[no_mangle]
pub fn skia_color_create(r: u8, g: u8, b: u8, a: u8) -> OwnedPtr<Color> {
    OwnedPtr::new(Color::from_argb(a, r, g, b))
}

#[no_mangle]
pub fn skia_color_create_argb(argb: u32) -> OwnedPtr<Color> {
    OwnedPtr::new(Color::new(argb))
}

#[no_mangle]
pub fn skia_color_get_red(color: BorrowedPtr<Color>) -> u8 {
    color.with_clone_ok(Color::r).or_log(0)
}

#[no_mangle]
pub fn skia_color_get_green(color: BorrowedPtr<Color>) -> u8 {
    color.with_clone_ok(Color::g).or_log(0)
}

#[no_mangle]
pub fn skia_color_get_blue(color: BorrowedPtr<Color>) -> u8 {
    color.with_clone_ok(Color::b).or_log(0)
}

#[no_mangle]
pub fn skia_color_get_alpha(color: BorrowedPtr<Color>) -> u8 {
    color.with_clone_ok(Color::a).or_log(0)
}

#[no_mangle]
pub fn skia_color_drop(color: OwnedPtr<Color>) {
    drop(color);
}

#[no_mangle]
pub fn skia_color_array_default() -> OwnedPtr<ArrayBox<Color>> {
    OwnedPtr::new(ArrayBox::new())
}

#[no_mangle]
pub fn skia_color_array_create_with(
    color: BorrowedPtr<Color>,
    amount: usize,
) -> OwnedPtr<ArrayBox<Color>> {
    color
        .with_clone_ok(|color| OwnedPtr::new(ArrayBox::from_vector(vec![color; amount])))
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_color_array_get_length(array: BorrowedPtr<ArrayBox<Color>>) -> usize {
    array.with_ref_ok(|array| array.length).or_log(0)
}

#[no_mangle]
pub fn skia_color_array_get_capacity(array: BorrowedPtr<ArrayBox<Color>>) -> usize {
    array.with_ref_ok(|array| array.capacity).or_log(0)
}

#[no_mangle]
pub fn skia_color_array_get_data(array: BorrowedPtr<ArrayBox<Color>>) -> *mut Color {
    array
        .with_ref_ok(|array| array.data)
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn skia_color_array_at(array: BorrowedPtr<ArrayBox<Color>>, index: usize) -> OwnedPtr<Color> {
    array
        .with_ref_ok(|array| OwnedPtr::new(array.at(index)))
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_color_array_at_put(
    mut array: BorrowedPtr<ArrayBox<Color>>,
    index: usize,
    color: BorrowedPtr<Color>,
) {
    color
        .with_ref(|color| array.with_mut_ok(|array| array.at_put(index, color.clone())))
        .log();
}

#[no_mangle]
pub fn skia_color_array_drop(ptr: OwnedPtr<ArrayBox<Color>>) {
    drop(ptr);
}

#[test]
pub fn test_skia_color_array() {
    let mut color = skia_color_default();
    let mut array_ptr = skia_color_array_create_with(color, 5);

    assert_eq!(!color.is_null(), true);
    assert_eq!(!array_ptr.is_null(), true);

    skia_color_array_drop(array_ptr);

    assert_eq!(!color.is_null(), true);
    assert_eq!(!array_ptr.is_null(), false);

    skia_color_drop(color);
    assert_eq!(!color.is_null(), false);
}
