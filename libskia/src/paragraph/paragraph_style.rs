use skia_safe::scalar;
use skia_safe::textlayout::{ParagraphStyle, TextStyle};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_paragraph_style_new() -> *mut ValueBox<ParagraphStyle> {
    let mut style = ParagraphStyle::new();
    style.set_apply_rounding_hack(false);
    style.set_replace_tab_characters(true);
    ValueBox::new(style).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_style_get_apply_rounding_hack(
    paragraph: *mut ValueBox<ParagraphStyle>,
) -> bool {
    paragraph
        .with_mut_ok(|style| style.apply_rounding_hack())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paragraph_style_set_apply_rounding_hack(
    paragraph: *mut ValueBox<ParagraphStyle>,
    apply_rounding_hack: bool,
) {
    paragraph
        .with_mut_ok(|style| {
            style.set_apply_rounding_hack(apply_rounding_hack);
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_style_get_text_style(
    paragraph_ptr: *mut ValueBox<ParagraphStyle>,
) -> *mut ValueBox<TextStyle> {
    paragraph_ptr
        .with_ref_ok(|style| ValueBox::new(style.text_style().clone()))
        .into_raw()
}

#[no_mangle]
pub fn skia_paragraph_style_set_text_style(
    paragraph_ptr: *mut ValueBox<ParagraphStyle>,
    text_style_ptr: *mut ValueBox<TextStyle>,
) {
    paragraph_ptr
        .with_mut_ok(|style| {
            text_style_ptr.with_ref_ok(|text_style| {
                style.set_text_style(text_style);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_style_get_height(paragraph_ptr: *mut ValueBox<ParagraphStyle>) -> scalar {
    paragraph_ptr
        .with_ref_ok(|style| style.height())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_style_set_height(
    paragraph_ptr: *mut ValueBox<ParagraphStyle>,
    height: scalar,
) {
    paragraph_ptr
        .with_mut_ok(|style| {
            style.set_height(height);
        })
        .log()
}

#[no_mangle]
pub fn skia_paragraph_style_set_max_lines(
    paragraph_ptr: *mut ValueBox<ParagraphStyle>,
    max_lines: usize,
) {
    paragraph_ptr
        .with_mut_ok(|style| {
            style.set_max_lines(if max_lines == usize::MAX {
                None
            } else {
                Some(max_lines)
            });
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_style_get_max_lines(paragraph_ptr: *mut ValueBox<ParagraphStyle>) -> usize {
    paragraph_ptr
        .with_ref_ok(|style| style.max_lines().unwrap_or(usize::MAX))
        .or_log(0)
}

#[no_mangle]
pub fn skia_paragraph_style_drop(ptr: *mut ValueBox<ParagraphStyle>) {
    ptr.release();
}
