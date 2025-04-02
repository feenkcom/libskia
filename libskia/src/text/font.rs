use array_box::ArrayBox;
use skia_safe::font::Edging;
use skia_safe::{
    scalar, Font, FontHinting, FontMetrics, GlyphId, Paint, Rect, TextEncoding, Typeface,
};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_font_default() -> *mut ValueBox<Font> {
    ValueBox::new(Font::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_from_typeface(
    typeface: *mut ValueBox<Typeface>,
    size: scalar,
) -> *mut ValueBox<Font> {
    typeface
        .with_clone_ok(|typeface| ValueBox::new(Font::from_typeface(typeface, size)))
        .into_raw()
}

#[no_mangle]
pub fn skia_font_is_force_auto_hinting(font: *mut ValueBox<Font>) -> bool {
    font.with_ref_ok(|font| font.is_force_auto_hinting())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_is_embedded_bitmaps(font: *mut ValueBox<Font>) -> bool {
    font.with_ref_ok(|font| font.is_embedded_bitmaps())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_is_subpixel(font: *mut ValueBox<Font>) -> bool {
    font.with_ref_ok(|font| font.is_subpixel()).or_log(false)
}

#[no_mangle]
pub fn skia_font_set_subpixel(font: *mut ValueBox<Font>, is_subpixel: bool) {
    font.with_mut_ok(|font| {
        font.set_subpixel(is_subpixel);
    })
    .log();
}

#[no_mangle]
pub fn skia_font_is_linear_metrics(font: *mut ValueBox<Font>) -> bool {
    font.with_ref_ok(|font| font.is_linear_metrics())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_is_embolden(font: *mut ValueBox<Font>) -> bool {
    font.with_ref_ok(|font| font.is_embolden()).or_log(false)
}

#[no_mangle]
pub fn skia_font_is_baseline_snap(font: *mut ValueBox<Font>) -> bool {
    font.with_ref_ok(|font| font.is_baseline_snap())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_get_edging(font: *mut ValueBox<Font>) -> Edging {
    font.with_ref_ok(|font| font.edging()).or_log(Edging::Alias)
}

#[no_mangle]
pub fn skia_font_set_edging(font: *mut ValueBox<Font>, font_edging: Edging) {
    font.with_mut_ok(|font| {
        font.set_edging(font_edging);
    })
    .log();
}

#[no_mangle]
pub fn skia_font_get_hinting(font: *mut ValueBox<Font>) -> FontHinting {
    font.with_ref_ok(|font| font.hinting())
        .or_log(FontHinting::None)
}

#[no_mangle]
pub fn skia_font_set_hinting(font: *mut ValueBox<Font>, font_hinting: FontHinting) {
    font.with_mut_ok(|font| {
        font.set_hinting(font_hinting);
    })
    .log();
}

#[no_mangle]
pub fn skia_font_get_typeface_or_default(font: *mut ValueBox<Font>) -> *mut ValueBox<Typeface> {
    font.with_ref_ok(|font| ValueBox::new(font.typeface()))
        .into_raw()
}

#[no_mangle]
pub fn skia_font_get_size(font: *mut ValueBox<Font>) -> scalar {
    font.with_ref_ok(|font| font.size()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_scale_x(font: *mut ValueBox<Font>) -> scalar {
    font.with_ref_ok(|font| font.scale_x()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_skew_x(font: *mut ValueBox<Font>) -> scalar {
    font.with_ref_ok(|font| font.skew_x()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_spacing(font: *mut ValueBox<Font>) -> scalar {
    font.with_ref_ok(|font| font.spacing()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_metrics(font: *mut ValueBox<Font>) -> *mut ValueBox<FontMetrics> {
    font.with_ref_ok(|font| ValueBox::new(font.metrics().1))
        .into_raw()
}

#[no_mangle]
pub fn skia_font_text_to_glyphs(
    font: *mut ValueBox<Font>,
    text_ptr: *mut ValueBox<StringBox>,
    _encoding: TextEncoding,
    glyphs_ptr: *mut ValueBox<ArrayBox<GlyphId>>,
    paint_ptr: *mut ValueBox<Paint>,
    bounds_ptr: *mut ValueBox<Rect>,
) -> scalar {
    font.with_ref(|font| {
        glyphs_ptr.with_mut(|glyphs| {
            text_ptr.with_ref_ok(|text| {
                let mut advance = 0.0;
                let glyphs_vec = font.text_to_glyphs_vec(text.as_str());
                if glyphs_vec.len() > 0 {
                    paint_ptr
                        .with_ref(|paint| {
                            bounds_ptr.with_mut_ok(|bounds| {
                                // this is faster than computing ourselves
                                let (text_advance, text_bounds) =
                                    font.measure_text(text.as_str(), Some(paint));
                                advance = text_advance;
                                bounds.set_ltrb(
                                    text_bounds.left,
                                    text_bounds.top,
                                    text_bounds.right,
                                    text_bounds.bottom,
                                );
                            })
                        })
                        .ok();
                    glyphs.set_vector(glyphs_vec)
                }
                advance
            })
        })
    })
    .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_measure_text(
    font: *mut ValueBox<Font>,
    text_ptr: *mut ValueBox<StringBox>,
    _encoding: TextEncoding,
    paint_ptr: *mut ValueBox<Paint>,
    bounds_ptr: *mut ValueBox<Rect>,
) -> scalar {
    font.with_ref(|font| {
        text_ptr.with_ref(|text| {
            paint_ptr.with_ref(|paint| {
                bounds_ptr.with_mut_ok(|bounds| {
                    let metrics = font.measure_text(text.as_str(), Some(paint));
                    bounds.set_ltrb(
                        metrics.1.left,
                        metrics.1.top,
                        metrics.1.right,
                        metrics.1.bottom,
                    );
                    metrics.0
                })
            })
        })
    })
    .or_log(0.0)
}

#[no_mangle]
pub fn skia_font_drop(ptr: *mut ValueBox<Font>) {
    ptr.release();
}
