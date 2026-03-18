use skia_safe::paint::{Cap, Join, Style};
use skia_safe::{BlendMode, Color, ImageFilter, Paint, PathEffect, Shader, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_default() -> OwnedPtr<Paint> {
    OwnedPtr::new(Paint::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_reset(mut paint: BorrowedPtr<Paint>) {
    paint
        .with_mut_ok(|paint| {
            paint.reset();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_is_anti_alias(paint: BorrowedPtr<Paint>) -> bool {
    paint
        .with_ref_ok(|paint| paint.is_anti_alias())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_anti_alias(mut paint: BorrowedPtr<Paint>, anti_alias: bool) {
    paint
        .with_mut_ok(|paint| {
            paint.set_anti_alias(anti_alias);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_is_dither(paint: BorrowedPtr<Paint>) -> bool {
    paint.with_ref_ok(|paint| paint.is_dither()).or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_dither(mut paint: BorrowedPtr<Paint>, dither: bool) {
    paint
        .with_mut_ok(|paint| {
            paint.set_dither(dither);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_style(paint: BorrowedPtr<Paint>) -> Style {
    paint.with_ref_ok(|paint| paint.style()).or_log(Style::Fill)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_style(mut paint: BorrowedPtr<Paint>, style: Style) {
    paint
        .with_mut_ok(|paint| {
            paint.set_style(style);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_rgba(mut paint: BorrowedPtr<Paint>, r: u8, g: u8, b: u8, a: u8) {
    paint
        .with_mut_ok(|paint| {
            paint.set_argb(a, r, g, b);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_alpha(mut paint: BorrowedPtr<Paint>, alpha: u8) {
    paint
        .with_mut_ok(|paint| {
            paint.set_alpha(alpha);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_alpha_f(mut paint: BorrowedPtr<Paint>, alpha: f32) {
    paint
        .with_mut_ok(|paint| {
            paint.set_alpha_f(alpha);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_alpha(paint: BorrowedPtr<Paint>) -> u8 {
    paint.with_ref_ok(|paint| paint.alpha()).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_alpha_f(paint: BorrowedPtr<Paint>) -> f32 {
    paint.with_ref_ok(|paint| paint.alpha_f()).or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_color(paint: BorrowedPtr<Paint>) -> OwnedPtr<Color> {
    paint
        .with_ref_ok(|paint| OwnedPtr::new(paint.color()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_stroke_width(paint: BorrowedPtr<Paint>) -> scalar {
    paint.with_ref_ok(|paint| paint.stroke_width()).or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_stroke_width(mut paint: BorrowedPtr<Paint>, width: scalar) {
    paint
        .with_mut_ok(|paint| {
            paint.set_stroke_width(width);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_blend_mode(paint: BorrowedPtr<Paint>) -> BlendMode {
    paint
        .with_ref_ok(|paint| paint.blend_mode_or(BlendMode::SrcOver))
        .or_log(BlendMode::Clear)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_blend_mode(mut paint: BorrowedPtr<Paint>, blend_mode: BlendMode) {
    paint
        .with_mut_ok(|paint| {
            paint.set_blend_mode(blend_mode);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_stroke_miter(paint: BorrowedPtr<Paint>) -> scalar {
    paint.with_ref_ok(|paint| paint.stroke_miter()).or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_stroke_miter(mut paint: BorrowedPtr<Paint>, stroke_miter: scalar) {
    paint
        .with_mut_ok(|paint| {
            paint.set_stroke_miter(stroke_miter);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_stroke_cap(paint: BorrowedPtr<Paint>) -> Cap {
    paint
        .with_ref_ok(|paint| paint.stroke_cap())
        .or_log(Cap::Butt)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_stroke_cap(mut paint: BorrowedPtr<Paint>, stroke_cap: Cap) {
    paint
        .with_mut_ok(|paint| {
            paint.set_stroke_cap(stroke_cap);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_stroke_join(paint: BorrowedPtr<Paint>) -> Join {
    paint
        .with_ref_ok(|paint| paint.stroke_join())
        .or_log(Join::Miter)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_stroke_join(mut paint: BorrowedPtr<Paint>, stroke_join: Join) {
    paint
        .with_mut_ok(|paint| {
            paint.set_stroke_join(stroke_join);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_shader(paint: BorrowedPtr<Paint>) -> OwnedPtr<Shader> {
    paint
        .with_ref_ok(|paint| match paint.shader() {
            None => OwnedPtr::null(),
            Some(shader) => OwnedPtr::new(shader),
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_shader(
    mut paint: BorrowedPtr<Paint>,
    shader: BorrowedPtr<Shader>,
) {
    paint
        .with_mut_ok(|paint| {
            let shader = shader.with_clone_ok(Some).unwrap_or(None);
            paint.set_shader(shader);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_image_filter(
    mut paint: BorrowedPtr<Paint>,
    image_filter: BorrowedPtr<ImageFilter>,
) {
    paint
        .with_mut_ok(|paint| {
            let image_filter = image_filter.with_clone_ok(Some).unwrap_or(None);
            paint.set_image_filter(image_filter);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_image_filter(paint: BorrowedPtr<Paint>) -> OwnedPtr<ImageFilter> {
    paint
        .with_ref_ok(|paint| match paint.image_filter() {
            None => OwnedPtr::null(),
            Some(image_filter) => OwnedPtr::new(image_filter),
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_has_image_filter(paint: BorrowedPtr<Paint>) -> bool {
    paint
        .with_ref_ok(|paint| paint.image_filter().is_some())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_set_path_effect(
    mut paint: BorrowedPtr<Paint>,
    path_effect: BorrowedPtr<PathEffect>,
) {
    paint
        .with_mut_ok(|paint| {
            let path_effect = path_effect.with_clone_ok(Some).unwrap_or(None);
            paint.set_path_effect(path_effect);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_get_path_effect(paint: BorrowedPtr<Paint>) -> OwnedPtr<PathEffect> {
    paint
        .with_ref_ok(|paint| match paint.path_effect() {
            None => OwnedPtr::null(),
            Some(path_effect) => OwnedPtr::new(path_effect),
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_has_path_effect(paint: BorrowedPtr<Paint>) -> bool {
    paint
        .with_ref_ok(|paint| paint.path_effect().is_some())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paint_drop(paint: OwnedPtr<Paint>) {
    drop(paint);
}
