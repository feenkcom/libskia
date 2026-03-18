use Skia::types::{skia_glyph_id_name, skia_scalar_name};
use string_box::StringBox;
use value_box::BorrowedPtr;

#[test]
fn scalar_name() {
    let mut string = StringBox::new();
    skia_scalar_name(BorrowedPtr::from_mut(&mut string));
    assert_eq!(string.to_string(), "f32");
}

#[test]
fn glyph_id_name() {
    let mut string = StringBox::new();
    skia_glyph_id_name(BorrowedPtr::from_mut(&mut string));
    assert_eq!(string.to_string(), "u16");
}
