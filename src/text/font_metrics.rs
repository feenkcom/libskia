use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{FontMetrics, scalar};

#[no_mangle]
pub fn skia_font_metrics_default() -> *mut ValueBox<FontMetrics> {
    ValueBox::new(FontMetrics::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_metrics_get_top(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.top)
}

#[no_mangle]
pub fn skia_font_metrics_get_ascent(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.ascent)
}

#[no_mangle]
pub fn skia_font_metrics_get_descent(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.descent)
}

#[no_mangle]
pub fn skia_font_metrics_get_bottom(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.bottom)
}

#[no_mangle]
pub fn skia_font_metrics_get_leading(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.leading)
}

#[no_mangle]
pub fn skia_font_metrics_get_avg_char_width(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.avg_char_width)
}

#[no_mangle]
pub fn skia_font_metrics_get_max_char_width(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.max_char_width)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_min(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.x_min)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_max(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.x_max)
}

#[no_mangle]
pub fn skia_font_metrics_get_x_height(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.x_height)
}

#[no_mangle]
pub fn skia_font_metrics_get_cap_height(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.cap_height)
}

#[no_mangle]
pub fn skia_font_metrics_get_underline_thickness(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.underline_thickness().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_get_underline_position(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.underline_position().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_get_strikeout_thickness(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.strikeout_thickness().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_get_strikeout_position(_ptr: *mut ValueBox<FontMetrics>) -> scalar {
   _ptr.with(|metrics| metrics.strikeout_position().unwrap_or(0.0))
}

#[no_mangle]
pub fn skia_font_metrics_drop(_ptr: *mut ValueBox<FontMetrics>) {
   _ptr.drop();
}