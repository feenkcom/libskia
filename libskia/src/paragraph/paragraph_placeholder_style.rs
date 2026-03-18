use skia_safe::scalar;
use skia_safe::textlayout::{PlaceholderAlignment, PlaceholderStyle};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_placeholder_style_default() -> OwnedPtr<PlaceholderStyle> {
    OwnedPtr::new(PlaceholderStyle::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_placeholder_style_set_width(
    mut placeholder_style: BorrowedPtr<PlaceholderStyle>,
    width: scalar,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.width = width;
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_placeholder_style_set_height(
    mut placeholder_style: BorrowedPtr<PlaceholderStyle>,
    height: scalar,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.height = height;
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_placeholder_style_set_alignment(
    mut placeholder_style: BorrowedPtr<PlaceholderStyle>,
    alignment: PlaceholderAlignment,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.alignment = alignment;
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_placeholder_style_set_baseline_offset(
    mut placeholder_style: BorrowedPtr<PlaceholderStyle>,
    baseline_offset: scalar,
) {
    placeholder_style
        .with_mut_ok(|placeholder| {
            placeholder.baseline_offset = baseline_offset;
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_placeholder_style_drop(
    placeholder_style: OwnedPtr<PlaceholderStyle>,
) {
    drop(placeholder_style);
}
