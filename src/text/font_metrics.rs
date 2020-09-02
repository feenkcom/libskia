use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::{scalar, FontMetrics};

#[no_mangle]
pub fn skia_font_metrics_default() -> *mut ValueBox<FontMetrics> {
    ValueBox::new(FontMetrics::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_metrics_get_top(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.top)
}

#[no_mangle]
pub fn skia_font_metrics_get_ascent(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.ascent)
}

#[no_mangle]
pub fn skia_font_metrics_get_descent(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.descent)
}

#[no_mangle]
pub fn skia_font_metrics_get_bottom(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.bottom)
}

#[no_mangle]
pub fn skia_font_metrics_get_leading(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.leading)
}

#[no_mangle]
pub fn skia_font_metrics_get_avg_char_width(
    font_metrics_ptr: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.avg_char_width)
}

#[no_mangle]
pub fn skia_font_metrics_get_max_char_width(
    font_metrics_ptr: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.max_char_width)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_min(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.x_min)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_max(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.x_max)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_height(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.x_height)
}

#[no_mangle]
pub fn skia_font_metrics_get_cap_height(font_metrics_ptr: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics_ptr.with_not_null_return(0.0, |metrics| metrics.cap_height)
}

#[no_mangle]
pub fn skia_font_metrics_get_underline_thickness(
    font_metrics_ptr: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics_ptr
        .with_not_null_return(0.0, |metrics| metrics.underline_thickness().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_get_underline_position(
    font_metrics_ptr: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics_ptr
        .with_not_null_return(0.0, |metrics| metrics.underline_position().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_get_strikeout_thickness(
    font_metrics_ptr: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics_ptr
        .with_not_null_return(0.0, |metrics| metrics.strikeout_thickness().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_get_strikeout_position(
    font_metrics_ptr: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics_ptr
        .with_not_null_return(0.0, |metrics| metrics.strikeout_position().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_drop(ptr: &mut *mut ValueBox<FontMetrics>) {
    drop!(ptr);
}
