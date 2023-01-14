use skia_safe::scalar;
use skia_safe::textlayout::{PlaceholderAlignment, PlaceholderStyle};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_paragraph_placeholder_style_default() -> *mut ValueBox<PlaceholderStyle> {
    ValueBox::new(PlaceholderStyle::default()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_width(
    placeholder_ptr: *mut ValueBox<PlaceholderStyle>,
    width: scalar,
) {
    placeholder_ptr.with_not_null(|placeholder| {
        placeholder.width = width;
    })
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_height(
    placeholder_ptr: *mut ValueBox<PlaceholderStyle>,
    height: scalar,
) {
    placeholder_ptr.with_not_null(|placeholder| {
        placeholder.height = height;
    })
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_alignment(
    placeholder_ptr: *mut ValueBox<PlaceholderStyle>,
    alignment: PlaceholderAlignment,
) {
    placeholder_ptr.with_not_null(|placeholder| {
        placeholder.alignment = alignment;
    })
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_set_baseline_offset(
    placeholder_ptr: *mut ValueBox<PlaceholderStyle>,
    baseline_offset: scalar,
) {
    placeholder_ptr.with_not_null(|placeholder| {
        placeholder.baseline_offset = baseline_offset;
    })
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_drop(ptr: *mut ValueBox<PlaceholderStyle>) {
    ptr.release();
}
