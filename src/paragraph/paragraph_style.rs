use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::textlayout::{ParagraphStyle, TextStyle};
use skia_safe::scalar;

#[no_mangle]
pub fn skia_paragraph_style_new() -> *mut ValueBox<ParagraphStyle> {
    ValueBox::new(ParagraphStyle::new()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_style_get_text_style(paragraph_ptr: *mut ValueBox<ParagraphStyle>) -> *mut ValueBox<TextStyle> {
    paragraph_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        ValueBox::new(style.text_style().clone()).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_style_set_text_style(paragraph_ptr: *mut ValueBox<ParagraphStyle>, text_style_ptr: *mut ValueBox<TextStyle>) {
    paragraph_ptr.with_not_null( |style| {
        text_style_ptr.with_not_null(|text_style| {
            style.set_text_style(text_style);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_style_get_height(paragraph_ptr: *mut ValueBox<ParagraphStyle>) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |style| {
        style.height()
    })
}

#[no_mangle]
pub fn skia_paragraph_style_set_height(paragraph_ptr: *mut ValueBox<ParagraphStyle>, height: scalar) {
    paragraph_ptr.with_not_null(|style| {
        style.set_height(height);
    })
}

#[no_mangle]
pub fn skia_paragraph_style_set_max_lines(paragraph_ptr: *mut ValueBox<ParagraphStyle>, max_lines: usize) {
    paragraph_ptr.with_not_null(|style| {
        style.set_max_lines(if max_lines == usize::max_value() { None } else { Some(max_lines) });
    })
}

#[no_mangle]
pub fn skia_paragraph_style_get_max_lines(paragraph_ptr: *mut ValueBox<ParagraphStyle>) -> usize {
    paragraph_ptr.with_not_null_return(0, |style| {
        style.max_lines().unwrap_or(usize::max_value())
    })
}

#[no_mangle]
pub fn skia_paragraph_style_drop(ptr: *mut ValueBox<ParagraphStyle>) {
    ptr.drop()
}