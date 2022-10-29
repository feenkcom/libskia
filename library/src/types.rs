extern crate typename;

use string_box::StringBox;
use typename::TypeName;
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_scalar_name(_ptr: *mut ValueBox<StringBox>) {
    _ptr.with_not_null(|string| string.set_string(skia_safe::scalar::type_name()))
}

#[no_mangle]
pub fn skia_glyph_id_name(_ptr: *mut ValueBox<StringBox>) {
    _ptr.with_not_null(|string| string.set_string(skia_safe::GlyphId::type_name()))
}

#[test]
fn scalar_name() {
    let _string_ptr = ValueBox::new(StringBox::new()).into_raw();
    skia_scalar_name(_string_ptr);
    _string_ptr.with_not_null(|string| {
        assert_eq!(string.to_string(), "f32");
    });
}

#[test]
fn glyph_id_name() {
    let _string_ptr = ValueBox::new(StringBox::new()).into_raw();
    skia_glyph_id_name(_string_ptr);
    _string_ptr.with_not_null(|string| {
        assert_eq!(string.to_string(), "u16");
    });
}
