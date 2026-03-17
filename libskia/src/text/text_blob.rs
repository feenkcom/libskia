use skia_safe::{Font, GlyphId, Point, TextBlob, TextBlobBuilder, TextEncoding};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_text_blob_default() -> OwnedPtr<TextBlob> {
    TextBlob::from_str("Text", &Font::default())
        .map(OwnedPtr::new)
        .unwrap_or_else(OwnedPtr::null)
}

#[no_mangle]
pub fn skia_text_blob_from_text(
    text: BorrowedPtr<StringBox>,
    _encoding: TextEncoding,
    font: BorrowedPtr<Font>,
) -> OwnedPtr<TextBlob> {
    text.with_ref(|text| {
        font.with_ref_ok(|font| {
            TextBlob::from_text(text.as_str(), font).map(|blob| OwnedPtr::new(blob))
        })
    })
    .or_log(None)
    .unwrap_or_default()
}

#[no_mangle]
pub fn skia_text_blob_from_glyphs(
    glyphs: *mut GlyphId,
    glyphs_length: usize,
    font: BorrowedPtr<Font>,
) -> OwnedPtr<TextBlob> {
    let glyphs = unsafe { std::slice::from_raw_parts(glyphs, glyphs_length) };

    font.with_ref_ok(|font| {
        let mut blob_builder: TextBlobBuilder = TextBlobBuilder::new();
        let allocated_glyphs: &mut [GlyphId] =
            blob_builder.alloc_run(font, glyphs_length, Point::new(0.0, 0.0), None);
        allocated_glyphs.copy_from_slice(glyphs);

        blob_builder.make().map(|blob| OwnedPtr::new(blob))
    })
    .or_log(None)
    .unwrap_or_default()
}

#[no_mangle]
pub fn skia_text_blob_drop(ptr: OwnedPtr<TextBlob>) {
    drop(ptr);
}
