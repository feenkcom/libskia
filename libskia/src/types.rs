extern crate typename;

use string_box::StringBox;
use typename::TypeName;
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
