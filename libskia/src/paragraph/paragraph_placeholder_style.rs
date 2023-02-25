use skia_safe::scalar;
use skia_safe::textlayout::{PlaceholderAlignment, PlaceholderStyle};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_paragraph_placeholder_style_default() -> *mut ValueBox<PlaceholderStyle> {
    value_box!(PlaceholderStyle::default()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_width(
    placeholder_style: *mut ValueBox<PlaceholderStyle>,
    width: scalar,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.width = width;
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_height(
    placeholder_style: *mut ValueBox<PlaceholderStyle>,
    height: scalar,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.height = height;
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_alignment(
    placeholder_style: *mut ValueBox<PlaceholderStyle>,
    alignment: PlaceholderAlignment,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.alignment = alignment;
        })
        .log()
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_baseline_offset(
    placeholder_style: *mut ValueBox<PlaceholderStyle>,
    baseline_offset: scalar,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.baseline_offset = baseline_offset;
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_drop(placeholder_style: *mut ValueBox<PlaceholderStyle>) {
    placeholder_style.release();
}
