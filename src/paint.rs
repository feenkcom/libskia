use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{Paint, FilterQuality, scalar, BlendMode, Color, Shader, ImageFilter};
use skia_safe::paint::{Style, Cap, Join};

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
pub fn skia_paint_get_filter_quality(_paint_ptr: *mut ValueBox<Paint>) -> FilterQuality {
    _paint_ptr.with(|paint| paint.filter_quality())
}

#[no_mangle]
pub fn skia_paint_set_filter_quality(_paint_ptr: *mut ValueBox<Paint>, quality: FilterQuality) {
    _paint_ptr.with(|paint| { paint.set_filter_quality(quality); });
}

#[no_mangle]
pub fn skia_paint_get_style(_paint_ptr: *mut ValueBox<Paint>) -> Style {
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
pub fn skia_paint_set_alpha(_paint_ptr: *mut ValueBox<Paint>, alpha: u8) {
    _paint_ptr.with(|paint| { paint.set_alpha(alpha); });
}

#[no_mangle]
pub fn skia_paint_set_alpha_f(_paint_ptr: *mut ValueBox<Paint>, alpha: f32) {
    _paint_ptr.with(|paint| { paint.set_alpha_f(alpha); });
}

#[no_mangle]
pub fn skia_paint_get_alpha(_paint_ptr: *mut ValueBox<Paint>) -> u8 {
    _paint_ptr.with(|paint| paint.alpha())
}

#[no_mangle]
pub fn skia_paint_get_alpha_f(_paint_ptr: *mut ValueBox<Paint>) -> f32 {
    _paint_ptr.with(|paint| paint.alpha_f())
}

#[no_mangle]
pub fn skia_paint_get_color(_paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<Color> {
    _paint_ptr.with(|paint| ValueBox::new(paint.color()).into_raw())
}

#[no_mangle]
pub fn skia_paint_get_stroke_width(_paint_ptr: *mut ValueBox<Paint>) -> scalar {
    _paint_ptr.with(|paint| paint.stroke_width())
}

#[no_mangle]
pub fn skia_paint_set_stroke_width(_paint_ptr: *mut ValueBox<Paint>, width: scalar) {
    _paint_ptr.with(|paint| { paint.set_stroke_width(width); });
}

#[no_mangle]
pub fn skia_paint_get_blend_mode(_paint_ptr: *mut ValueBox<Paint>) -> BlendMode {
    _paint_ptr.with(|paint| paint.blend_mode())
}

#[no_mangle]
pub fn skia_paint_set_blend_mode(_paint_ptr: *mut ValueBox<Paint>, blend_mode: BlendMode) {
    _paint_ptr.with(|paint| { paint.set_blend_mode(blend_mode); });
}

#[no_mangle]
pub fn skia_paint_get_stroke_miter(_paint_ptr: *mut ValueBox<Paint>) -> scalar {
    _paint_ptr.with(|paint| paint.stroke_miter())
}

#[no_mangle]
pub fn skia_paint_set_stroke_miter(_paint_ptr: *mut ValueBox<Paint>, stroke_miter: scalar) {
    _paint_ptr.with(|paint| { paint.set_stroke_miter(stroke_miter); });
}

#[no_mangle]
pub fn skia_paint_get_stroke_cap(_paint_ptr: *mut ValueBox<Paint>) -> Cap {
    _paint_ptr.with(|paint| paint.stroke_cap())
}

#[no_mangle]
pub fn skia_paint_set_stroke_cap(_paint_ptr: *mut ValueBox<Paint>, stroke_cap: Cap) {
    _paint_ptr.with(|paint| { paint.set_stroke_cap(stroke_cap); });
}

#[no_mangle]
pub fn skia_paint_get_stroke_join(_paint_ptr: *mut ValueBox<Paint>) -> Join {
    _paint_ptr.with(|paint| paint.stroke_join())
}

#[no_mangle]
pub fn skia_paint_set_stroke_join(_paint_ptr: *mut ValueBox<Paint>, stroke_join: Join) {
    _paint_ptr.with(|paint| { paint.set_stroke_join(stroke_join); });
}

#[no_mangle]
pub fn skia_paint_get_shader(_paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<Shader> {
    _paint_ptr.with(|paint| match paint.shader() {
        None => { std::ptr::null_mut() },
        Some(shader) => { ValueBox::new(shader).into_raw() },
    })
}

#[no_mangle]
pub fn skia_paint_set_shader(_paint_ptr: *mut ValueBox<Paint>, _shader_ptr: *mut ValueBox<Shader>) {
    _paint_ptr.with(|paint| match _shader_ptr.as_option() {
        None => { paint.set_shader(None); },
        Some(mut _ptr) => { _ptr.with_value_consumed(|shader| { paint.set_shader(Some(shader)); } ) },
    });
}

#[no_mangle]
pub fn skia_paint_set_image_filter(_paint_ptr: *mut ValueBox<Paint>, _image_filter_ptr: *mut ValueBox<ImageFilter>) {
    _paint_ptr.with(|paint| match _image_filter_ptr.as_option() {
        None => { paint.set_image_filter(None); },
        Some(mut _ptr) => { _ptr.with_value_consumed(|image_filter| { paint.set_image_filter(Some(image_filter)); } ) },
    });
}

#[no_mangle]
pub fn skia_paint_get_image_filter(_paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<ImageFilter> {
    _paint_ptr.with(|paint| match paint.image_filter() {
        None => { std::ptr::null_mut() },
        Some(image_filter) => { ValueBox::new(image_filter).into_raw() },
    })
}

#[no_mangle]
pub fn skia_paint_drop(_ptr: *mut ValueBox<Paint>) {
    _ptr.drop();
}