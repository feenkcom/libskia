use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::textlayout::TextStyle;
use skia_safe::{scalar, Color, Paint};

#[no_mangle]
pub fn skia_paragraph_text_style_new() -> *mut ValueBox<TextStyle> {
    ValueBox::new(TextStyle::new()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_font_size(text_style_ptr: *mut ValueBox<TextStyle>) -> scalar {
    text_style_ptr.with_not_null_return(0.0, |style| {
        style.font_size()
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_word_spacing(text_style_ptr: *mut ValueBox<TextStyle>) -> scalar {
    text_style_ptr.with_not_null_return(0.0, |style| {
        style.word_spacing()
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_letter_spacing(text_style_ptr: *mut ValueBox<TextStyle>) -> scalar {
    text_style_ptr.with_not_null_return(0.0, |style| {
        style.letter_spacing()
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_color(text_style_ptr: *mut ValueBox<TextStyle>) -> *mut ValueBox<Color> {
    text_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        ValueBox::new(style.color()).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_color(text_style_ptr: *mut ValueBox<TextStyle>, color_ptr: *mut ValueBox<Color>) {
    text_style_ptr.with_not_null( |style| {
        color_ptr.with_not_null_value(|color| {
            style.set_color(color);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_foreground(text_style_ptr: *mut ValueBox<TextStyle>) -> *mut ValueBox<Paint> {
    text_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        match style.foreground() {
            None => { std::ptr::null_mut() },
            Some(paint) => { ValueBox::new(paint.clone()).into_raw() },
        }
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_foreground(text_style_ptr: *mut ValueBox<TextStyle>, paint_ptr: *mut ValueBox<Paint>) {
    text_style_ptr.with_not_null( |style| {
        paint_ptr.with_not_null_value(|paint| {
            style.set_foreground_color(paint);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_background(text_style_ptr: *mut ValueBox<TextStyle>) -> *mut ValueBox<Paint> {
    text_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        match style.background() {
            None => { std::ptr::null_mut() },
            Some(paint) => { ValueBox::new(paint.clone()).into_raw() },
        }
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_background(text_style_ptr: *mut ValueBox<TextStyle>, paint_ptr: *mut ValueBox<Paint>) {
    text_style_ptr.with_not_null( |style| {
        paint_ptr.with_not_null_value(|paint| {
            style.set_background_color(paint);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_drop(ptr: *mut ValueBox<TextStyle>) {
    ptr.drop()
}