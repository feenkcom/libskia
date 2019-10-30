use skia_safe::ColorSpace;
use boxer::CBox;

#[no_mangle]
pub fn skia_color_space_new_srgb() -> *mut ColorSpace {
    CBox::into_raw(ColorSpace::new_srgb())
}

#[no_mangle]
pub fn skia_color_space_drop(_ptr: *mut ColorSpace) {
    CBox::drop(_ptr);
}

#[no_mangle]
pub fn skia_color_space_is_srgb(_ptr: *mut ColorSpace) -> bool {
    CBox::with_optional_raw(_ptr, |option| {
        match option {
            None => { false },
            Some(color_space) => { color_space.is_srgb() },
        }
    })
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
    let _color_space_ptr: *mut ColorSpace = std::ptr::null_mut();
    assert_eq!(_color_space_ptr.is_null(), true);
    assert_eq!(skia_color_space_is_srgb(_color_space_ptr), false);
    skia_color_space_drop(_color_space_ptr);
}