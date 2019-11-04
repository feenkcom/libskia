use skia_safe::{TextBlob, Font};
use boxer::boxes::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_text_blob_default() -> *mut ValueBox<TextBlob> {
    match TextBlob::from_str("Text", &Font::default()) {
        None => { std::ptr::null_mut() },
        Some(text_blob) => {  ValueBox::new(text_blob).into_raw() },
    }
}

#[no_mangle]
pub fn skia_text_blob_drop(_ptr: *mut ValueBox<TextBlob>) {
    _ptr.drop();
}