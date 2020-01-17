use boxer::array::BoxerArray;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::{BoxerString, BoxerStringPointer};
use skia_safe::{Font, GlyphId, Point, TextBlob, TextBlobBuilder, TextEncoding};

#[no_mangle]
pub fn skia_text_blob_default() -> *mut ValueBox<TextBlob> {
    match TextBlob::from_str("Text", &Font::default()) {
        None => std::ptr::null_mut(),
        Some(text_blob) => ValueBox::new(text_blob).into_raw(),
    }
}

#[no_mangle]
pub fn skia_text_blob_from_text(
    _text_ptr: *mut BoxerString,
    encoding: TextEncoding,
    _font_ptr: *mut ValueBox<Font>,
) -> *mut ValueBox<TextBlob> {
    _text_ptr.with(|text|
        _font_ptr.with_not_null_return(std::ptr::null_mut(),|font| {
           match TextBlob::from_text(text.to_slice_u8(), encoding, font) {
                None => {
                    if cfg!(debug_assertions) {
                        eprintln!("[skia_text_blob_from_text] Failed to create TextBlob from {:?} #{:?} encoded as {:?} with font {:?}", text.to_string(), text.to_slice_u8(), encoding, font.typeface_or_default().family_name());
                    }
                    std::ptr::null_mut() },
                Some(text_blob) => {  ValueBox::new(text_blob).into_raw() },
            }
        }))
}

#[no_mangle]
pub fn skia_text_blob_from_glyphs(
    _glyphs_ptr: *mut ValueBox<BoxerArray<GlyphId>>,
    _font_ptr: *mut ValueBox<Font>,
) -> *mut ValueBox<TextBlob> {
    _glyphs_ptr.with_not_null_return(std::ptr::null_mut(), |glyphs|
        _font_ptr.with_not_null_return(std::ptr::null_mut(), |font| {
            let mut blob_builder = TextBlobBuilder::new();
            let allocated_glyphs = blob_builder.alloc_run(font, glyphs.length,Point::new(0.0,0.0), None);

            for (index, glyph) in glyphs.to_slice().iter().enumerate() {
                allocated_glyphs[index] = *glyph;
            }
            match  blob_builder.make() {
                None => {
                    if cfg!(debug_assertions) {
                        eprintln!("[skia_text_blob_from_glyphs] Failed to create TextBlob from glyphs with font {:?}", font.typeface_or_default().family_name());
                    }
                    std::ptr::null_mut() },
                Some(text_blob) => {  ValueBox::new(text_blob).into_raw() },
            }
        }))
}

#[no_mangle]
pub fn skia_text_blob_drop(_ptr: *mut ValueBox<TextBlob>) {
    _ptr.drop();
}
