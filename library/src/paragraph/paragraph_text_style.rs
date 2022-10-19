use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::textlayout::{Decoration, TextStyle};
use skia_safe::{scalar, Color, FontStyle, Paint};

#[no_mangle]
pub fn skia_paragraph_text_style_new() -> *mut ValueBox<TextStyle> {
    ValueBox::new(TextStyle::new()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_font_size(text_style_ptr: *mut ValueBox<TextStyle>) -> scalar {
    text_style_ptr.with_not_null_return(0.0, |style| style.font_size())
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_font_size(
    text_style_ptr: *mut ValueBox<TextStyle>,
    font_size: scalar,
) {
    text_style_ptr.with_not_null(|style| {
        style.set_font_size(font_size);
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_word_spacing(
    text_style_ptr: *mut ValueBox<TextStyle>,
) -> scalar {
    text_style_ptr.with_not_null_return(0.0, |style| style.word_spacing())
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_word_spacing(
    text_style_ptr: *mut ValueBox<TextStyle>,
    word_spacing: scalar,
) {
    text_style_ptr.with_not_null(|style| {
        style.set_word_spacing(word_spacing);
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_letter_spacing(
    text_style_ptr: *mut ValueBox<TextStyle>,
) -> scalar {
    text_style_ptr.with_not_null_return(0.0, |style| style.letter_spacing())
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_letter_spacing(
    text_style_ptr: *mut ValueBox<TextStyle>,
    letter_spacing: scalar,
) {
    text_style_ptr.with_not_null(|style| {
        style.set_letter_spacing(letter_spacing);
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_color(
    text_style_ptr: *mut ValueBox<TextStyle>,
) -> *mut ValueBox<Color> {
    text_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        ValueBox::new(style.color()).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_color(
    text_style_ptr: *mut ValueBox<TextStyle>,
    color_ptr: *mut ValueBox<Color>,
) {
    text_style_ptr.with_not_null(|style| {
        color_ptr.with_not_null_value(|color| {
            style.set_color(color);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_foreground(
    text_style_ptr: *mut ValueBox<TextStyle>,
) -> *mut ValueBox<Paint> {
    text_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| match style.foreground() {
        None => std::ptr::null_mut(),
        Some(paint) => ValueBox::new(paint.clone()).into_raw(),
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_foreground(
    text_style_ptr: *mut ValueBox<TextStyle>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    text_style_ptr.with_not_null(|style| {
        paint_ptr.with_not_null_value(|paint| {
            style.set_foreground_color(paint);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_background(
    text_style_ptr: *mut ValueBox<TextStyle>,
) -> *mut ValueBox<Paint> {
    text_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| match style.background() {
        None => std::ptr::null_mut(),
        Some(paint) => ValueBox::new(paint.clone()).into_raw(),
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_background(
    text_style_ptr: *mut ValueBox<TextStyle>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    text_style_ptr.with_not_null(|style| {
        paint_ptr.with_not_null_value(|paint| {
            style.set_background_color(paint);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_font_style(
    text_style_ptr: *mut ValueBox<TextStyle>,
    font_style_ptr: *mut ValueBox<FontStyle>,
) {
    text_style_ptr.with_not_null(|text_style| {
        font_style_ptr.with_not_null_value(|font_style| {
            text_style.set_font_style(font_style);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_font_family(
    text_style_ptr: *mut ValueBox<TextStyle>,
    font_family_ptr: *mut ValueBox<BoxerString>,
) {
    text_style_ptr.with_not_null(|text_style| {
        font_family_ptr.with_not_null(|font_family| {
            text_style.set_font_families(&[font_family.as_str()]);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_decoration(
    text_style_ptr: *mut ValueBox<TextStyle>,
    decoration_ptr: *mut ValueBox<Decoration>,
) {
    text_style_ptr.with_not_null(|text_style| {
        decoration_ptr.with_not_null(|decoration| {
            text_style.decoration_mut().clone_from(decoration);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_drop(ptr: *mut ValueBox<TextStyle>) {
    ptr.release();
}
