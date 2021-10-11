use boxer::function;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::paint::{Cap, Join, Style};
use skia_safe::{scalar, BlendMode, Color, ImageFilter, Paint, PathEffect, Shader};

#[no_mangle]
pub fn skia_paint_default() -> *mut ValueBox<Paint> {
    ValueBox::new(Paint::default()).into_raw()
}

#[no_mangle]
pub fn skia_paint_reset(paint_ptr: *mut ValueBox<Paint>) {
    paint_ptr.with_not_null(|paint| {
        paint.reset();
    });
}

#[no_mangle]
pub fn skia_paint_is_anti_alias(paint_ptr: *mut ValueBox<Paint>) -> bool {
    paint_ptr.with_not_null_return(false, |paint| paint.is_anti_alias())
}

#[no_mangle]
pub fn skia_paint_set_anti_alias(paint_ptr: *mut ValueBox<Paint>, anti_alias: bool) {
    paint_ptr.with_not_null(|paint| {
        paint.set_anti_alias(anti_alias);
    });
}

#[no_mangle]
pub fn skia_paint_is_dither(paint_ptr: *mut ValueBox<Paint>) -> bool {
    paint_ptr.with_not_null_return(false, |paint| paint.is_dither())
}

#[no_mangle]
pub fn skia_paint_set_dither(paint_ptr: *mut ValueBox<Paint>, dither: bool) {
    paint_ptr.with_not_null(|paint| {
        paint.set_dither(dither);
    });
}

#[no_mangle]
pub fn skia_paint_get_style(paint_ptr: *mut ValueBox<Paint>) -> Style {
    paint_ptr.with_not_null_return(Style::Fill, |paint| paint.style())
}

#[no_mangle]
pub fn skia_paint_set_style(paint_ptr: *mut ValueBox<Paint>, style: Style) {
    paint_ptr.with_not_null(|paint| {
        paint.set_style(style);
    });
}

#[no_mangle]
pub fn skia_paint_set_rgba(paint_ptr: *mut ValueBox<Paint>, r: u8, g: u8, b: u8, a: u8) {
    paint_ptr.with_not_null(|paint| {
        paint.set_argb(a, r, g, b);
    });
}

#[no_mangle]
pub fn skia_paint_set_alpha(paint_ptr: *mut ValueBox<Paint>, alpha: u8) {
    paint_ptr.with_not_null(|paint| {
        paint.set_alpha(alpha);
    });
}

#[no_mangle]
pub fn skia_paint_set_alpha_f(paint_ptr: *mut ValueBox<Paint>, alpha: f32) {
    paint_ptr.with_not_null(|paint| {
        paint.set_alpha_f(alpha);
    });
}

#[no_mangle]
pub fn skia_paint_get_alpha(paint_ptr: *mut ValueBox<Paint>) -> u8 {
    paint_ptr.with_not_null_return(0, |paint| paint.alpha())
}

#[no_mangle]
pub fn skia_paint_get_alpha_f(paint_ptr: *mut ValueBox<Paint>) -> f32 {
    paint_ptr.with_not_null_return(0.0, |paint| paint.alpha_f())
}

#[no_mangle]
pub fn skia_paint_get_color(paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<Color> {
    paint_ptr.with_not_null_return(std::ptr::null_mut(), |paint| {
        ValueBox::new(paint.color()).into_raw()
    })
}

#[no_mangle]
pub fn skia_paint_get_stroke_width(paint_ptr: *mut ValueBox<Paint>) -> scalar {
    paint_ptr.with_not_null_return(0.0, |paint| paint.stroke_width())
}

#[no_mangle]
pub fn skia_paint_set_stroke_width(paint_ptr: *mut ValueBox<Paint>, width: scalar) {
    paint_ptr.with_not_null(|paint| {
        paint.set_stroke_width(width);
    });
}

#[no_mangle]
pub fn skia_paint_get_blend_mode(paint_ptr: *mut ValueBox<Paint>) -> BlendMode {
    paint_ptr.with_not_null_return(BlendMode::Clear, |paint| paint.blend_mode())
}

#[no_mangle]
pub fn skia_paint_set_blend_mode(paint_ptr: *mut ValueBox<Paint>, blend_mode: BlendMode) {
    paint_ptr.with_not_null(|paint| {
        paint.set_blend_mode(blend_mode);
    });
}

#[no_mangle]
pub fn skia_paint_get_stroke_miter(paint_ptr: *mut ValueBox<Paint>) -> scalar {
    paint_ptr.with_not_null_return(0.0, |paint| paint.stroke_miter())
}

#[no_mangle]
pub fn skia_paint_set_stroke_miter(paint_ptr: *mut ValueBox<Paint>, stroke_miter: scalar) {
    paint_ptr.with_not_null(|paint| {
        paint.set_stroke_miter(stroke_miter);
    });
}

#[no_mangle]
pub fn skia_paint_get_stroke_cap(paint_ptr: *mut ValueBox<Paint>) -> Cap {
    paint_ptr.with_not_null_return(Cap::Butt, |paint| paint.stroke_cap())
}

#[no_mangle]
pub fn skia_paint_set_stroke_cap(paint_ptr: *mut ValueBox<Paint>, stroke_cap: Cap) {
    paint_ptr.with_not_null(|paint| {
        paint.set_stroke_cap(stroke_cap);
    });
}

#[no_mangle]
pub fn skia_paint_get_stroke_join(paint_ptr: *mut ValueBox<Paint>) -> Join {
    paint_ptr.with_not_null_return(Join::Miter, |paint| paint.stroke_join())
}

#[no_mangle]
pub fn skia_paint_set_stroke_join(paint_ptr: *mut ValueBox<Paint>, stroke_join: Join) {
    paint_ptr.with_not_null(|paint| {
        paint.set_stroke_join(stroke_join);
    });
}

#[no_mangle]
pub fn skia_paint_get_shader(paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<Shader> {
    paint_ptr.with_not_null_return(std::ptr::null_mut(), |paint| match paint.shader() {
        None => std::ptr::null_mut(),
        Some(shader) => ValueBox::new(shader).into_raw(),
    })
}

#[no_mangle]
pub fn skia_paint_set_shader(paint_ptr: *mut ValueBox<Paint>, shader_ptr: *mut ValueBox<Shader>) {
    paint_ptr.with_not_null(|paint| {
        let shader = shader_ptr.with_not_null_value_return(None, |shader| Some(shader));
        paint.set_shader(shader);
    });
}

#[no_mangle]
pub fn skia_paint_set_image_filter(
    paint_ptr: *mut ValueBox<Paint>,
    image_filter_ptr: *mut ValueBox<ImageFilter>,
) {
    paint_ptr.with_not_null(|paint| {
        let image_filter =
            image_filter_ptr.with_not_null_value_return(None, |image_filter| Some(image_filter));
        paint.set_image_filter(image_filter);
    });
}

#[no_mangle]
pub fn skia_paint_get_image_filter(paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<ImageFilter> {
    paint_ptr.with_not_null_return(std::ptr::null_mut(), |paint| match paint.image_filter() {
        None => std::ptr::null_mut(),
        Some(image_filter) => ValueBox::new(image_filter).into_raw(),
    })
}

#[no_mangle]
pub fn skia_paint_has_image_filter(paint_ptr: *mut ValueBox<Paint>) -> bool {
    paint_ptr.with_not_null_return(false, |paint| match paint.image_filter() {
        None => false,
        Some(_) => true,
    })
}

#[no_mangle]
pub fn skia_paint_set_path_effect(
    paint_ptr: *mut ValueBox<Paint>,
    path_effect_ptr: *mut ValueBox<PathEffect>,
) {
    paint_ptr.with_not_null(|paint| {
        let path_effect =
            path_effect_ptr.with_not_null_value_return(None, |path_effect| Some(path_effect));
        paint.set_path_effect(path_effect);
    });
}

#[no_mangle]
pub fn skia_paint_get_path_effect(paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<PathEffect> {
    paint_ptr.with_not_null_return(std::ptr::null_mut(), |paint| match paint.path_effect() {
        None => std::ptr::null_mut(),
        Some(path_effect) => ValueBox::new(path_effect).into_raw(),
    })
}

#[no_mangle]
pub fn skia_paint_has_path_effect(paint_ptr: *mut ValueBox<Paint>) -> bool {
    paint_ptr.with_not_null_return(false, |paint| match paint.path_effect() {
        None => false,
        Some(_) => true,
    })
}

#[no_mangle]
pub fn skia_paint_drop(ptr: &mut *mut ValueBox<Paint>) {
    drop!(ptr);
}
