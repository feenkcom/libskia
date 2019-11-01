use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{Paint, FilterQuality, scalar, BlendMode, Color};
use skia_safe::paint::Style;

#[no_mangle]
pub fn skia_paint_default() -> *mut ValueBox<Paint> {
    ValueBox::new(Paint::default()).into_raw()
}

#[no_mangle]
pub fn skia_paint_reset(_paint_ptr: *mut ValueBox<Paint>) {
    _paint_ptr.with(|paint| { paint.reset(); });
}

#[no_mangle]
pub fn skia_paint_is_anti_alias(_paint_ptr: *mut ValueBox<Paint>) -> bool {
    _paint_ptr.with(|paint| paint.is_anti_alias())
}

#[no_mangle]
pub fn skia_paint_set_anti_alias(_paint_ptr: *mut ValueBox<Paint>, anti_alias: bool) {
    _paint_ptr.with(|paint| { paint.set_anti_alias(anti_alias); });
}

#[no_mangle]
pub fn skia_paint_is_dither(_paint_ptr: *mut ValueBox<Paint>) -> bool {
    _paint_ptr.with(|paint| paint.is_dither())
}

#[no_mangle]
pub fn skia_paint_set_dither(_paint_ptr: *mut ValueBox<Paint>, dither: bool) {
    _paint_ptr.with(|paint| { paint.set_dither(dither); });
}

#[no_mangle]
pub fn skia_paint_filter_quality(_paint_ptr: *mut ValueBox<Paint>) -> FilterQuality {
    _paint_ptr.with(|paint| paint.filter_quality())
}

#[no_mangle]
pub fn skia_paint_set_filter_quality(_paint_ptr: *mut ValueBox<Paint>, quality: FilterQuality) {
    _paint_ptr.with(|paint| { paint.set_filter_quality(quality); });
}

#[no_mangle]
pub fn skia_paint_style(_paint_ptr: *mut ValueBox<Paint>) -> Style {
    _paint_ptr.with(|paint| paint.style())
}

#[no_mangle]
pub fn skia_paint_set_style(_paint_ptr: *mut ValueBox<Paint>, style: Style) {
    _paint_ptr.with(|paint| { paint.set_style(style); });
}

#[no_mangle]
pub fn skia_paint_set_rgba(_paint_ptr: *mut ValueBox<Paint>, r: u8, g: u8, b: u8, a: u8) {
    _paint_ptr.with(|paint| { paint.set_argb(a, r, g, b); });
}

#[no_mangle]
pub fn skia_paint_get_color(_paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<Color> {
    _paint_ptr.with(|paint| ValueBox::new(paint.color()).into_raw())
}

#[no_mangle]
pub fn skia_paint_stroke_width(_paint_ptr: *mut ValueBox<Paint>) -> scalar {
    _paint_ptr.with(|paint| paint.stroke_width())
}

#[no_mangle]
pub fn skia_paint_set_stroke_width(_paint_ptr: *mut ValueBox<Paint>, width: scalar) {
    _paint_ptr.with(|paint| { paint.set_stroke_width(width); });
}

#[no_mangle]
pub fn skia_paint_blend_mode(_paint_ptr: *mut ValueBox<Paint>) -> BlendMode {
    _paint_ptr.with(|paint| paint.blend_mode())
}

#[no_mangle]
pub fn skia_paint_set_blend_mode(_paint_ptr: *mut ValueBox<Paint>, blend_mode: BlendMode) {
    _paint_ptr.with(|paint| { paint.set_blend_mode(blend_mode); });
}

#[no_mangle]
pub fn skia_paint_drop(_ptr: *mut ValueBox<Paint>) {
    _ptr.drop();
}