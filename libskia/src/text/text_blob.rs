use skia_safe::{Font, GlyphId, Point, TextBlob, TextBlobBuilder, TextEncoding};
use string_box::StringBox;
use value_box::{ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_text_blob_default() -> *mut ValueBox<TextBlob> {
    TextBlob::from_str("Text", &Font::default())
        .map(|blob| value_box!(blob).into_raw())
        .unwrap_or_else(|| std::ptr::null_mut())
}

#[no_mangle]
pub fn skia_text_blob_from_text(
    text: *mut ValueBox<StringBox>,
    encoding: TextEncoding,
    font: *mut ValueBox<Font>,
) -> *mut ValueBox<TextBlob> {
    text.with_ref(|text| {
        font.with_ref_ok(|font| {
            TextBlob::from_text(text.as_str(), font).map(|blob| value_box!(blob))
        })
    })
    .into_raw()
}

#[no_mangle]
pub fn skia_text_blob_from_glyphs(
    glyphs: *mut GlyphId,
    glyphs_length: usize,
    font: *mut ValueBox<Font>,
) -> *mut ValueBox<TextBlob> {
    let glyphs = unsafe { std::slice::from_raw_parts(glyphs, glyphs_length) };

    font.with_ref_ok(|font| {
        let mut blob_builder: TextBlobBuilder = TextBlobBuilder::new();
        let allocated_glyphs: &mut [GlyphId] =
            blob_builder.alloc_run(font, glyphs_length, Point::new(0.0, 0.0), None);
        allocated_glyphs.copy_from_slice(glyphs);

        blob_builder.make().map(|blob| value_box!(blob))
    })
    .into_raw()
}

#[no_mangle]
pub fn skia_text_blob_drop(ptr: *mut ValueBox<TextBlob>) {
    ptr.release();
}
