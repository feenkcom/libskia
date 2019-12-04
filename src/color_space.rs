use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::ColorSpace;

#[no_mangle]
pub fn skia_color_space_new_srgb() -> *mut ValueBox<ColorSpace> {
    ValueBox::new(ColorSpace::new_srgb()).into_raw()
}

#[no_mangle]
pub fn skia_color_space_drop(_ptr: *mut ValueBox<ColorSpace>) {
    _ptr.drop();
}

#[no_mangle]
pub fn skia_color_space_is_srgb(_ptr: *mut ValueBox<ColorSpace>) -> bool {
    _ptr.with_not_null_return(false, |color_space| color_space.is_srgb())
}

#[test]
fn color_space_new_srgb() {
    let _color_space_ptr = skia_color_space_new_srgb();
    assert_eq!(_color_space_ptr.is_null(), false);
    assert_eq!(skia_color_space_is_srgb(_color_space_ptr), true);
    skia_color_space_drop(_color_space_ptr);
}

#[test]
fn color_space_is_srgb_for_null() {
    let _color_space_ptr: *mut ValueBox<ColorSpace> = std::ptr::null_mut();
    assert_eq!(_color_space_ptr.is_null(), true);
    assert_eq!(skia_color_space_is_srgb(_color_space_ptr), false);
    skia_color_space_drop(_color_space_ptr);
}
