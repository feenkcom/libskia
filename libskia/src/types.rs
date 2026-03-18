extern crate typename;

use string_box::StringBox;
use typename::TypeName;
#[cfg(test)]
use value_box::OwnedPtr;
use value_box::{BorrowedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_scalar_name(mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(skia_safe::scalar::type_name()))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_glyph_id_name(mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(skia_safe::GlyphId::type_name()))
        .log();
}

#[test]
fn scalar_name() {
    let string = OwnedPtr::new(StringBox::new());
    skia_scalar_name(string);
    string
        .with_ref_ok(|string| {
            assert_eq!(string.to_string(), "f32");
        })
        .log();
}

#[test]
fn glyph_id_name() {
    let string = OwnedPtr::new(StringBox::new());
    skia_glyph_id_name(string);
    string
        .with_ref_ok(|string| {
            assert_eq!(string.to_string(), "u16");
        })
        .log();
}
