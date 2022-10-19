use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer};
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
    text_ptr: *mut ValueBox<BoxerString>,
    encoding: TextEncoding,
    font_ptr: *mut ValueBox<Font>,
) -> *mut ValueBox<TextBlob> {
    text_ptr.with_not_null_return(std::ptr::null_mut(), |text|
        font_ptr.with_not_null_return(std::ptr::null_mut(),|font| {
           match TextBlob::from_text(text.as_bytes(), encoding, font) {
                None => {
                    if cfg!(debug_assertions) {
                        eprintln!("[skia_text_blob_from_text] Failed to create TextBlob from {:?} #{:?} encoded as {:?} with font {:?}", text.to_string(), text.as_bytes(), encoding, font.typeface_or_default().family_name());
                    }
                    std::ptr::null_mut() },
                Some(text_blob) => {  ValueBox::new(text_blob).into_raw() },
            }
        }))
}

#[no_mangle]
pub fn skia_text_blob_from_glyphs(
    glyphs_ptr: *mut GlyphId,
    glyphs_length: usize,
    font_ptr: *mut ValueBox<Font>,
) -> *mut ValueBox<TextBlob> {
    let glyphs = unsafe { std::slice::from_raw_parts(glyphs_ptr, glyphs_length) };

    font_ptr.with_not_null_return(std::ptr::null_mut(), |font| {
        let mut blob_builder = TextBlobBuilder::new();
        let allocated_glyphs = blob_builder.alloc_run(font, glyphs_length,Point::new(0.0,0.0), None);
        allocated_glyphs.copy_from_slice(glyphs);

        match  blob_builder.make() {
            None => {
                if cfg!(debug_assertions) {
                    eprintln!("[skia_text_blob_from_glyphs] Failed to create TextBlob from glyphs with font {:?}", font.typeface_or_default().family_name());
                }
                std::ptr::null_mut() },
            Some(text_blob) => {  ValueBox::new(text_blob).into_raw() },
        }
    })
}

#[no_mangle]
pub fn skia_text_blob_drop(ptr: *mut ValueBox<TextBlob>) {
    ptr.release();
}
