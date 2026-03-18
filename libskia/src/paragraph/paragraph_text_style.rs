use skia_safe::textlayout::{Decoration, TextStyle};
use skia_safe::{Color, FontStyle, Paint, scalar};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_new() -> OwnedPtr<TextStyle> {
    OwnedPtr::new(TextStyle::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_get_font_size(text_style: BorrowedPtr<TextStyle>) -> scalar {
    text_style
        .with_ref_ok(|style| style.font_size())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_font_size(
    mut text_style: BorrowedPtr<TextStyle>,
    font_size: scalar,
) {
    text_style
        .with_mut_ok(|style| {
            style.set_font_size(font_size);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_get_word_spacing(text_style: BorrowedPtr<TextStyle>) -> scalar {
    text_style
        .with_ref_ok(|style| style.word_spacing())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_word_spacing(
    mut text_style: BorrowedPtr<TextStyle>,
    word_spacing: scalar,
) {
    text_style
        .with_mut_ok(|style| {
            style.set_word_spacing(word_spacing);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_get_letter_spacing(text_style: BorrowedPtr<TextStyle>) -> scalar {
    text_style
        .with_clone_ok(|style| style.letter_spacing())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_letter_spacing(
    mut text_style: BorrowedPtr<TextStyle>,
    letter_spacing: scalar,
) {
    text_style
        .with_mut_ok(|style| {
            style.set_letter_spacing(letter_spacing);
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_get_color(text_style: BorrowedPtr<TextStyle>) -> OwnedPtr<Color> {
    text_style
        .with_clone_ok(|style| OwnedPtr::new(style.color()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_color(
    mut text_style: BorrowedPtr<TextStyle>,
    color_ptr: BorrowedPtr<Color>,
) {
    text_style
        .with_mut_ok(|style| {
            color_ptr.with_clone_ok(|color| {
                style.set_color(color);
            })
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_get_foreground(
    text_style: BorrowedPtr<TextStyle>,
) -> OwnedPtr<Paint> {
    text_style
        .with_ref_ok(|text_style| OwnedPtr::new(text_style.foreground()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_foreground(
    mut text_style: BorrowedPtr<TextStyle>,
    paint: BorrowedPtr<Paint>,
) {
    paint
        .with_ref(|paint| {
            text_style.with_mut_ok(|text_style| {
                text_style.set_foreground_paint(&paint);
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_get_background(
    text_style: BorrowedPtr<TextStyle>,
) -> OwnedPtr<Paint> {
    text_style
        .with_ref_ok(|text_style| OwnedPtr::new(text_style.background()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_background(
    mut text_style: BorrowedPtr<TextStyle>,
    paint: BorrowedPtr<Paint>,
) {
    paint
        .with_ref(|paint| {
            text_style.with_mut_ok(|text_style| {
                text_style.set_background_paint(paint);
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_font_style(
    mut text_style: BorrowedPtr<TextStyle>,
    font_style_ptr: BorrowedPtr<FontStyle>,
) {
    text_style
        .with_mut_ok(|text_style| {
            font_style_ptr.with_clone_ok(|font_style| {
                text_style.set_font_style(font_style);
            })
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_font_family(
    mut text_style: BorrowedPtr<TextStyle>,
    mut font_family_ptr: BorrowedPtr<StringBox>,
) {
    text_style
        .with_mut_ok(|text_style| {
            font_family_ptr.with_mut_ok(|font_family| {
                text_style.set_font_families(&[font_family.as_str()]);
            })
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_set_decoration(
    mut text_style: BorrowedPtr<TextStyle>,
    decoration_ptr: BorrowedPtr<Decoration>,
) {
    text_style
        .with_mut_ok(|text_style| {
            decoration_ptr.with_ref_ok(|decoration| {
                text_style.set_decoration(decoration);
            })
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_text_style_drop(ptr: OwnedPtr<TextStyle>) {
    drop(ptr);
}
