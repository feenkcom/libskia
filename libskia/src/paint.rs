use skia_safe::paint::{Cap, Join, Style};
use skia_safe::{scalar, BlendMode, Color, ImageFilter, Paint, PathEffect, Shader};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_paint_default() -> *mut ValueBox<Paint> {
    ValueBox::new(Paint::default()).into_raw()
}

#[no_mangle]
pub fn skia_paint_reset(paint_ptr: *mut ValueBox<Paint>) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.reset();
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_is_anti_alias(paint_ptr: *mut ValueBox<Paint>) -> bool {
    paint_ptr
        .with_ref_ok(|paint| paint.is_anti_alias())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paint_set_anti_alias(paint_ptr: *mut ValueBox<Paint>, anti_alias: bool) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_anti_alias(anti_alias);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_is_dither(paint_ptr: *mut ValueBox<Paint>) -> bool {
    paint_ptr
        .with_ref_ok(|paint| paint.is_dither())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paint_set_dither(paint_ptr: *mut ValueBox<Paint>, dither: bool) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_dither(dither);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_style(paint_ptr: *mut ValueBox<Paint>) -> Style {
    paint_ptr
        .with_ref_ok(|paint| paint.style())
        .or_log(Style::Fill)
}

#[no_mangle]
pub fn skia_paint_set_style(paint_ptr: *mut ValueBox<Paint>, style: Style) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_style(style);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_set_rgba(paint_ptr: *mut ValueBox<Paint>, r: u8, g: u8, b: u8, a: u8) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_argb(a, r, g, b);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_set_alpha(paint_ptr: *mut ValueBox<Paint>, alpha: u8) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_alpha(alpha);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_set_alpha_f(paint_ptr: *mut ValueBox<Paint>, alpha: f32) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_alpha_f(alpha);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_alpha(paint_ptr: *mut ValueBox<Paint>) -> u8 {
    paint_ptr.with_ref_ok(|paint| paint.alpha()).or_log(0)
}

#[no_mangle]
pub fn skia_paint_get_alpha_f(paint_ptr: *mut ValueBox<Paint>) -> f32 {
    paint_ptr.with_ref_ok(|paint| paint.alpha_f()).or_log(0.0)
}

#[no_mangle]
pub fn skia_paint_get_color(paint_ptr: *mut ValueBox<Paint>) -> *mut ValueBox<Color> {
    paint_ptr
        .with_ref_ok(|paint| ValueBox::new(paint.color()))
        .into_raw()
}

#[no_mangle]
pub fn skia_paint_get_stroke_width(paint_ptr: *mut ValueBox<Paint>) -> scalar {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_width())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paint_set_stroke_width(paint_ptr: *mut ValueBox<Paint>, width: scalar) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_width(width);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_blend_mode(paint_ptr: *mut ValueBox<Paint>) -> BlendMode {
    paint_ptr
        .with_ref_ok(|paint| paint.blend_mode())
        .or_log(BlendMode::Clear)
}

#[no_mangle]
pub fn skia_paint_set_blend_mode(paint_ptr: *mut ValueBox<Paint>, blend_mode: BlendMode) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_blend_mode(blend_mode);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_stroke_miter(paint_ptr: *mut ValueBox<Paint>) -> scalar {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_miter())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paint_set_stroke_miter(paint_ptr: *mut ValueBox<Paint>, stroke_miter: scalar) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_miter(stroke_miter);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_stroke_cap(paint_ptr: *mut ValueBox<Paint>) -> Cap {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_cap())
        .or_log(Cap::Butt)
}

#[no_mangle]
pub fn skia_paint_set_stroke_cap(paint_ptr: *mut ValueBox<Paint>, stroke_cap: Cap) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_cap(stroke_cap);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_stroke_join(paint_ptr: *mut ValueBox<Paint>) -> Join {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_join())
        .or_log(Join::Miter)
}

#[no_mangle]
pub fn skia_paint_set_stroke_join(paint_ptr: *mut ValueBox<Paint>, stroke_join: Join) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_join(stroke_join);
        })
        .log();
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
    paint_ptr
        .with_ref_ok(|paint| paint.image_filter().is_some())
        .or_log(false)
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
    paint_ptr
        .with_ref_ok(|paint| paint.path_effect().is_some())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paint_drop(ptr: *mut ValueBox<Paint>) {
    ptr.release();
}
