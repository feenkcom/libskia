use Skia::color_space::{
    skia_color_space_drop, skia_color_space_is_srgb, skia_color_space_new_srgb,
};
use skia_safe::ColorSpace;
use value_box::{BorrowedPtr, OwnedPtr};

#[test]
fn color_space_new_srgb() {
    let color_space = skia_color_space_new_srgb();
    assert!(!color_space.is_null());
    color_space
        .with_value_ok(|color_space| {
            assert!(color_space.is_srgb());
        })
        .unwrap();
}

#[test]
fn color_space_is_srgb_for_null() {
    let color_space = BorrowedPtr::<ColorSpace>::null();
    assert!(color_space.is_null());
    assert!(!skia_color_space_is_srgb(color_space));
}

#[test]
fn color_space_drop_accepts_owned_pointer() {
    skia_color_space_drop(OwnedPtr::new(ColorSpace::new_srgb()));
}
