use crate::value_box_compat::*;
use skia_safe::paint::{Cap, Join, Style};
use skia_safe::{scalar, BlendMode, Color, ImageFilter, Paint, PathEffect, Shader};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_paint_default() -> OwnedPtr<Paint> {
    OwnedPtr::new(Paint::default()).into_raw()
}

#[no_mangle]
pub fn skia_paint_reset(mut paint_ptr: BorrowedPtr<Paint>) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.reset();
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_is_anti_alias(paint_ptr: BorrowedPtr<Paint>) -> bool {
    paint_ptr
        .with_ref_ok(|paint| paint.is_anti_alias())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paint_set_anti_alias(mut paint_ptr: BorrowedPtr<Paint>, anti_alias: bool) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_anti_alias(anti_alias);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_is_dither(paint_ptr: BorrowedPtr<Paint>) -> bool {
    paint_ptr
        .with_ref_ok(|paint| paint.is_dither())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paint_set_dither(mut paint_ptr: BorrowedPtr<Paint>, dither: bool) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_dither(dither);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_style(paint_ptr: BorrowedPtr<Paint>) -> Style {
    paint_ptr
        .with_ref_ok(|paint| paint.style())
        .or_log(Style::Fill)
}

#[no_mangle]
pub fn skia_paint_set_style(mut paint_ptr: BorrowedPtr<Paint>, style: Style) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_style(style);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_set_rgba(mut paint_ptr: BorrowedPtr<Paint>, r: u8, g: u8, b: u8, a: u8) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_argb(a, r, g, b);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_set_alpha(mut paint_ptr: BorrowedPtr<Paint>, alpha: u8) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_alpha(alpha);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_set_alpha_f(mut paint_ptr: BorrowedPtr<Paint>, alpha: f32) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_alpha_f(alpha);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_alpha(paint_ptr: BorrowedPtr<Paint>) -> u8 {
    paint_ptr.with_ref_ok(|paint| paint.alpha()).or_log(0)
}

#[no_mangle]
pub fn skia_paint_get_alpha_f(paint_ptr: BorrowedPtr<Paint>) -> f32 {
    paint_ptr.with_ref_ok(|paint| paint.alpha_f()).or_log(0.0)
}

#[no_mangle]
pub fn skia_paint_get_color(paint_ptr: BorrowedPtr<Paint>) -> OwnedPtr<Color> {
    paint_ptr
        .with_ref_ok(|paint| OwnedPtr::new(paint.color()))
        .into_raw()
}

#[no_mangle]
pub fn skia_paint_get_stroke_width(paint_ptr: BorrowedPtr<Paint>) -> scalar {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_width())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paint_set_stroke_width(mut paint_ptr: BorrowedPtr<Paint>, width: scalar) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_width(width);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_blend_mode(paint_ptr: BorrowedPtr<Paint>) -> BlendMode {
    paint_ptr
        .with_ref_ok(|paint| paint.blend_mode())
        .or_log(BlendMode::Clear)
}

#[no_mangle]
pub fn skia_paint_set_blend_mode(mut paint_ptr: BorrowedPtr<Paint>, blend_mode: BlendMode) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_blend_mode(blend_mode);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_stroke_miter(paint_ptr: BorrowedPtr<Paint>) -> scalar {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_miter())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paint_set_stroke_miter(mut paint_ptr: BorrowedPtr<Paint>, stroke_miter: scalar) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_miter(stroke_miter);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_stroke_cap(paint_ptr: BorrowedPtr<Paint>) -> Cap {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_cap())
        .or_log(Cap::Butt)
}

#[no_mangle]
pub fn skia_paint_set_stroke_cap(mut paint_ptr: BorrowedPtr<Paint>, stroke_cap: Cap) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_cap(stroke_cap);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_stroke_join(paint_ptr: BorrowedPtr<Paint>) -> Join {
    paint_ptr
        .with_ref_ok(|paint| paint.stroke_join())
        .or_log(Join::Miter)
}

#[no_mangle]
pub fn skia_paint_set_stroke_join(mut paint_ptr: BorrowedPtr<Paint>, stroke_join: Join) {
    paint_ptr
        .with_mut_ok(|paint| {
            paint.set_stroke_join(stroke_join);
        })
        .log();
}

#[no_mangle]
pub fn skia_paint_get_shader(paint_ptr: BorrowedPtr<Paint>) -> OwnedPtr<Shader> {
    paint_ptr.with_not_null_return(OwnedPtr::null(), |paint| match paint.shader() {
        None => OwnedPtr::null(),
        Some(shader) => OwnedPtr::new(shader),
    })
}

#[no_mangle]
pub fn skia_paint_set_shader(paint_ptr: BorrowedPtr<Paint>, shader_ptr: BorrowedPtr<Shader>) {
    paint_ptr.with_not_null(|paint| {
        let shader = shader_ptr.with_not_null_value_return(None, |shader| Some(shader));
        paint.set_shader(shader);
    });
}

#[no_mangle]
pub fn skia_paint_set_image_filter(
    paint_ptr: BorrowedPtr<Paint>,
    image_filter_ptr: BorrowedPtr<ImageFilter>,
) {
    paint_ptr.with_not_null(|paint| {
        let image_filter =
            image_filter_ptr.with_not_null_value_return(None, |image_filter| Some(image_filter));
        paint.set_image_filter(image_filter);
    });
}

#[no_mangle]
pub fn skia_paint_get_image_filter(paint_ptr: BorrowedPtr<Paint>) -> OwnedPtr<ImageFilter> {
    paint_ptr.with_not_null_return(OwnedPtr::null(), |paint| match paint.image_filter() {
        None => OwnedPtr::null(),
        Some(image_filter) => OwnedPtr::new(image_filter),
    })
}

#[no_mangle]
pub fn skia_paint_has_image_filter(paint_ptr: BorrowedPtr<Paint>) -> bool {
    paint_ptr
        .with_ref_ok(|paint| paint.image_filter().is_some())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paint_set_path_effect(
    paint_ptr: BorrowedPtr<Paint>,
    path_effect_ptr: BorrowedPtr<PathEffect>,
) {
    paint_ptr.with_not_null(|paint| {
        let path_effect =
            path_effect_ptr.with_not_null_value_return(None, |path_effect| Some(path_effect));
        paint.set_path_effect(path_effect);
    });
}

#[no_mangle]
pub fn skia_paint_get_path_effect(paint_ptr: BorrowedPtr<Paint>) -> OwnedPtr<PathEffect> {
    paint_ptr.with_not_null_return(OwnedPtr::null(), |paint| match paint.path_effect() {
        None => OwnedPtr::null(),
        Some(path_effect) => OwnedPtr::new(path_effect),
    })
}

#[no_mangle]
pub fn skia_paint_has_path_effect(paint_ptr: BorrowedPtr<Paint>) -> bool {
    paint_ptr
        .with_ref_ok(|paint| paint.path_effect().is_some())
        .or_log(false)
}

#[no_mangle]
pub fn skia_paint_drop(mut ptr: OwnedPtr<Paint>) {
    ptr.release();
}
