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
    CBox::with_raw(_ptr, |color_space| color_space.is_srgb())
}

#[test]
fn color_space_new_srgb() {
    let _color_space_ptr = skia_color_space_new_srgb();
    assert_eq!(_color_space_ptr.is_null(), false);
    assert_eq!(skia_color_space_is_srgb(_color_space_ptr), true);
    assert_eq!(skia_color_space_is_srgb(_color_space_ptr), true);
    assert_eq!(skia_color_space_is_srgb(_color_space_ptr), true);
    assert_eq!(skia_color_space_is_srgb(_color_space_ptr), true);
    CBox::with_raw(_color_space_ptr, |color_space| println!("{:?}", color_space.serialize().as_bytes()));
    skia_color_space_drop(_color_space_ptr);
    let color_space = unsafe { CBox::from_raw(_color_space_ptr) };
    std::mem::drop(color_space);
//    CBox::with_raw(_color_space_ptr, |color_space| println!("{:?}", color_space.serialize().as_bytes()));
//    CBox::with_raw(_color_space_ptr, |color_space| println!("{:?}", color_space.serialize().as_bytes()));
}