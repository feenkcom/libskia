extern crate typename;

use super::*;
use self::typename::TypeName;
use boxer::string::BoxerString;
use boxer::CBox;

#[no_mangle]
pub fn skia_scalar_name(_ptr: *mut BoxerString) {
    CBox::with_raw(_ptr, |string| {
        string.set_string(skia_safe::scalar::type_name());
    })
}

#[no_mangle]
pub fn skia_glyph_id_name(_ptr: *mut BoxerString) {
    CBox::with_raw(_ptr, |string| {
        string.set_string(skia_safe::GlyphId::type_name());
    })
}

#[test]
fn scalar_name() {
    let _string_ptr = CBox::into_raw(BoxerString::default());
    skia_scalar_name(_string_ptr);
    let string = unsafe { CBox::from_raw(_string_ptr) };
    assert_eq!(string.to_string(), "f32");
}

#[test]
fn glyph_id_name() {
    let _string_ptr = CBox::into_raw(BoxerString::default());
    skia_glyph_id_name(_string_ptr);
    let string = unsafe { CBox::from_raw(_string_ptr) };
    assert_eq!(string.to_string(), "u16");
}