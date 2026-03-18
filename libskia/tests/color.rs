use Skia::color::{
    skia_color_array_create_with, skia_color_array_drop, skia_color_default, skia_color_drop,
};
use skia_safe::Color;
use value_box::BorrowedPtr;

#[test]
fn skia_color_array() {
    let color = Color::default();
    let array = skia_color_array_create_with(BorrowedPtr::from_ref(&color), 5);

    assert!(!array.is_null());
    array
        .with_value_ok(|array| {
            assert_eq!(array.length, 5);
            assert_eq!(array.capacity, 5);
        })
        .unwrap();
}

#[test]
fn color_drop_accepts_owned_pointer() {
    skia_color_drop(skia_color_default());
}

#[test]
fn color_array_drop_accepts_owned_pointer() {
    let color = Color::default();
    let array = skia_color_array_create_with(BorrowedPtr::from_ref(&color), 1);
    skia_color_array_drop(array);
}
