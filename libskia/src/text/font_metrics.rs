use skia_safe::{scalar, FontMetrics};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_font_metrics_default() -> *mut ValueBox<FontMetrics> {
    ValueBox::new(FontMetrics::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_metrics_get_top(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics.with_ref_ok(|metrics| metrics.top).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_ascent(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.ascent)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_descent(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.descent)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_bottom(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.bottom)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_leading(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.leading)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_avg_char_width(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.avg_char_width)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_max_char_width(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.max_char_width)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_min(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.x_min)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_max(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.x_max)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_height(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.x_height)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_cap_height(font_metrics: *mut ValueBox<FontMetrics>) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.cap_height)
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_underline_thickness(
    font_metrics: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.underline_thickness().unwrap_or(0.0))
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_underline_position(
    font_metrics: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.underline_position().unwrap_or(0.0))
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_strikeout_thickness(
    font_metrics: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.strikeout_thickness().unwrap_or(0.0))
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_get_strikeout_position(
    font_metrics: *mut ValueBox<FontMetrics>,
) -> scalar {
    font_metrics
        .with_ref_ok(|metrics| metrics.strikeout_position().unwrap_or(0.0))
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_metrics_drop(ptr: *mut ValueBox<FontMetrics>) {
    ptr.release();
}
