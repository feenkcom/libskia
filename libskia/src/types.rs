extern crate typename;

use string_box::StringBox;
use typename::TypeName;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_scalar_name(string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(skia_safe::scalar::type_name()))
        .log();
}

#[no_mangle]
pub fn skia_glyph_id_name(string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(skia_safe::GlyphId::type_name()))
        .log();
}

#[test]
fn scalar_name() {
    let string_ptr = ValueBox::new(StringBox::new()).into_raw();
    skia_scalar_name(string_ptr);
    string_ptr
        .with_ref_ok(|string| {
            assert_eq!(string.to_string(), "f32");
        })
        .log();
}

#[test]
fn glyph_id_name() {
    let string_ptr = ValueBox::new(StringBox::new()).into_raw();
    skia_glyph_id_name(string_ptr);
    string_ptr
        .with_ref_ok(|string| {
            assert_eq!(string.to_string(), "u16");
        })
        .log();
}
