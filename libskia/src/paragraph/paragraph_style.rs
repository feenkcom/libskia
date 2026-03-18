use skia_safe::scalar;
use skia_safe::textlayout::{ParagraphStyle, TextStyle};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_new() -> OwnedPtr<ParagraphStyle> {
    let mut style = ParagraphStyle::new();
    style.set_apply_rounding_hack(false);
    style.set_replace_tab_characters(true);
    OwnedPtr::new(style)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_get_apply_rounding_hack(
    mut paragraph: BorrowedPtr<ParagraphStyle>,
) -> bool {
    paragraph
        .with_mut_ok(|style| style.apply_rounding_hack())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_set_apply_rounding_hack(
    mut paragraph: BorrowedPtr<ParagraphStyle>,
    apply_rounding_hack: bool,
) {
    paragraph
        .with_mut_ok(|style| {
            style.set_apply_rounding_hack(apply_rounding_hack);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_get_text_style(
    paragraph_ptr: BorrowedPtr<ParagraphStyle>,
) -> OwnedPtr<TextStyle> {
    paragraph_ptr
        .with_ref_ok(|style| OwnedPtr::new(style.text_style().clone()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_set_text_style(
    mut paragraph_ptr: BorrowedPtr<ParagraphStyle>,
    text_style_ptr: BorrowedPtr<TextStyle>,
) {
    paragraph_ptr
        .with_mut_ok(|style| {
            text_style_ptr.with_ref_ok(|text_style| {
                style.set_text_style(text_style);
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_get_height(paragraph_ptr: BorrowedPtr<ParagraphStyle>) -> scalar {
    paragraph_ptr
        .with_ref_ok(|style| style.height())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_set_height(
    mut paragraph_ptr: BorrowedPtr<ParagraphStyle>,
    height: scalar,
) {
    paragraph_ptr
        .with_mut_ok(|style| {
            style.set_height(height);
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_set_max_lines(
    mut paragraph_ptr: BorrowedPtr<ParagraphStyle>,
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

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_get_max_lines(paragraph_ptr: BorrowedPtr<ParagraphStyle>) -> usize {
    paragraph_ptr
        .with_ref_ok(|style| style.max_lines().unwrap_or(usize::MAX))
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_style_drop(ptr: OwnedPtr<ParagraphStyle>) {
    drop(ptr);
}
