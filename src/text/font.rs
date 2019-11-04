use skia_safe::{Font, FontEdging, FontHinting, Typeface, scalar, TextEncoding, GlyphId, Rect, Paint, FontMetrics};
use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::array::{BoxerArrayU8, BoxerArray};

#[no_mangle]
pub fn skia_font_default() -> *mut ValueBox<Font> {
    ValueBox::new(Font::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_is_force_auto_hinting(_ptr: *mut ValueBox<Font>) -> bool {
    _ptr.with(|font| font.is_force_auto_hinting())
}

#[no_mangle]
pub fn skia_font_is_embedded_bitmaps(_ptr: *mut ValueBox<Font>) -> bool {
    _ptr.with(|font| font.is_embedded_bitmaps())
}

#[no_mangle]
pub fn skia_font_is_subpixel(_ptr: *mut ValueBox<Font>) -> bool {
    _ptr.with(|font| font.is_subpixel())
}

#[no_mangle]
pub fn skia_font_is_linear_metrics(_ptr: *mut ValueBox<Font>) -> bool {
    _ptr.with(|font| font.is_linear_metrics())
}

#[no_mangle]
pub fn skia_font_is_embolden(_ptr: *mut ValueBox<Font>) -> bool {
    _ptr.with(|font| font.is_embolden())
}

#[no_mangle]
pub fn skia_font_is_baseline_snap(_ptr: *mut ValueBox<Font>) -> bool {
    _ptr.with(|font| font.is_baseline_snap())
}

#[no_mangle]
pub fn skia_font_get_edging(_ptr: *mut ValueBox<Font>) -> FontEdging {
    _ptr.with(|font| font.edging())
}

#[no_mangle]
pub fn skia_font_get_hinting(_ptr: *mut ValueBox<Font>) -> FontHinting {
    _ptr.with(|font| font.hinting())
}

#[no_mangle]
pub fn skia_font_get_typeface_or_default(_ptr: *mut ValueBox<Font>) -> *mut ValueBox<Typeface> {
    _ptr.with(|font| ValueBox::new(font.typeface_or_default()).into_raw())
}

#[no_mangle]
pub fn skia_font_get_size(_ptr: *mut ValueBox<Font>) -> scalar {
    _ptr.with(|font| font.size())
}

#[no_mangle]
pub fn skia_font_get_scale_x(_ptr: *mut ValueBox<Font>) -> scalar {
    _ptr.with(|font| font.scale_x())
}

#[no_mangle]
pub fn skia_font_get_skew_x(_ptr: *mut ValueBox<Font>) -> scalar {
    _ptr.with(|font| font.skew_x())
}

#[no_mangle]
pub fn skia_font_get_spacing(_ptr: *mut ValueBox<Font>) -> scalar {
    _ptr.with(|font| font.spacing())
}

#[no_mangle]
pub fn skia_font_get_metrics(_ptr: *mut ValueBox<Font>) -> *mut ValueBox<FontMetrics> {
    _ptr.with(|font| ValueBox::new(font.metrics().1).into_raw())
}

#[no_mangle]
pub fn skia_font_text_to_glyphs(_ptr: *mut ValueBox<Font>, _text_ptr: *mut ValueBox<BoxerArrayU8>, encoding: TextEncoding, _glyphs_ptr: *mut ValueBox<BoxerArray<GlyphId>>) {
    _ptr.with(|font|
        _text_ptr.with(|text|
            _glyphs_ptr.with(|glyphs| {
                glyphs.set_vector(font.text_to_glyphs_vec(text.to_slice(), encoding))
            })));
}

#[no_mangle]
pub fn skia_font_measure_text(_ptr: *mut ValueBox<Font>, _text_ptr: *mut ValueBox<BoxerArrayU8>, encoding: TextEncoding, _paint_ptr: *mut ValueBox<Paint>, _bounds_ptr: *mut ValueBox<Rect>) -> scalar {
    _ptr.with(|font|
        _text_ptr.with(|text|
            _paint_ptr.with(|paint|
                _bounds_ptr.with(|bounds| {
                    let metrics = font.measure_text(text.to_slice(), encoding, Some(paint));
                    bounds.set_ltrb(metrics.1.left, metrics.1.top, metrics.1.right, metrics.1.bottom);
                    metrics.0
                }))))
}

#[no_mangle]
pub fn skia_font_drop(_ptr: *mut ValueBox<Font>) {
    _ptr.drop();
}