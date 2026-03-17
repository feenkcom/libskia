use crate::value_box_compat::*;
use skia_safe::textlayout::{Decoration, TextStyle};
use skia_safe::{scalar, Color, FontStyle, Paint};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_paragraph_text_style_new() -> OwnedPtr<TextStyle> {
    OwnedPtr::new(TextStyle::new()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_font_size(text_style: BorrowedPtr<TextStyle>) -> scalar {
    text_style
        .with_ref_ok(|style| style.font_size())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_font_size(
    mut text_style: BorrowedPtr<TextStyle>,
    font_size: scalar,
) {
    text_style
        .with_mut_ok(|style| {
            style.set_font_size(font_size);
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_word_spacing(text_style: BorrowedPtr<TextStyle>) -> scalar {
    text_style
        .with_ref_ok(|style| style.word_spacing())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_word_spacing(
    mut text_style: BorrowedPtr<TextStyle>,
    word_spacing: scalar,
) {
    text_style
        .with_mut_ok(|style| {
            style.set_word_spacing(word_spacing);
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_letter_spacing(text_style: BorrowedPtr<TextStyle>) -> scalar {
    text_style.with_not_null_return(0.0, |style| style.letter_spacing())
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_letter_spacing(
    text_style: BorrowedPtr<TextStyle>,
    letter_spacing: scalar,
) {
    text_style.with_not_null(|style| {
        style.set_letter_spacing(letter_spacing);
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_color(text_style: BorrowedPtr<TextStyle>) -> OwnedPtr<Color> {
    text_style.with_not_null_return(OwnedPtr::null(), |style| OwnedPtr::new(style.color()))
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_color(
    text_style: BorrowedPtr<TextStyle>,
    color_ptr: BorrowedPtr<Color>,
) {
    text_style.with_not_null(|style| {
        color_ptr.with_not_null_value(|color| {
            style.set_color(color);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_foreground(
    text_style: BorrowedPtr<TextStyle>,
) -> OwnedPtr<Paint> {
    text_style
        .with_ref_ok(|text_style| OwnedPtr::new(text_style.foreground()))
        .into_raw()
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_foreground(
    mut text_style: BorrowedPtr<TextStyle>,
    paint: BorrowedPtr<Paint>,
) {
    paint
        .with_ref(|paint| {
            text_style.with_mut_ok(|text_style| {
                text_style.set_foreground_color(&paint);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_text_style_get_background(
    text_style: BorrowedPtr<TextStyle>,
) -> OwnedPtr<Paint> {
    text_style
        .with_ref_ok(|text_style| OwnedPtr::new(text_style.background()))
        .into_raw()
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_background(
    mut text_style: BorrowedPtr<TextStyle>,
    paint: BorrowedPtr<Paint>,
) {
    paint
        .with_ref(|paint| {
            text_style.with_mut_ok(|text_style| {
                text_style.set_background_color(paint);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_font_style(
    text_style: BorrowedPtr<TextStyle>,
    font_style_ptr: BorrowedPtr<FontStyle>,
) {
    text_style.with_not_null(|text_style| {
        font_style_ptr.with_not_null_value(|font_style| {
            text_style.set_font_style(font_style);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_font_family(
    text_style: BorrowedPtr<TextStyle>,
    font_family_ptr: BorrowedPtr<StringBox>,
) {
    text_style.with_not_null(|text_style| {
        font_family_ptr.with_not_null(|font_family| {
            text_style.set_font_families(&[font_family.as_str()]);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_set_decoration(
    text_style: BorrowedPtr<TextStyle>,
    decoration_ptr: BorrowedPtr<Decoration>,
) {
    text_style.with_not_null(|text_style| {
        decoration_ptr.with_not_null(|decoration| {
            text_style.decoration_mut().clone_from(decoration);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_text_style_drop(mut ptr: OwnedPtr<TextStyle>) {
    ptr.release();
}
