use array_box::ArrayBox;
use skia_safe::font::Edging;
use skia_safe::{
    scalar, Font, FontHinting, FontMetrics, GlyphId, Paint, Rect, TextEncoding, Typeface,
};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_font_default() -> OwnedPtr<Font> {
    OwnedPtr::new(Font::default())
}

#[no_mangle]
pub fn skia_font_from_typeface(typeface: BorrowedPtr<Typeface>, size: scalar) -> OwnedPtr<Font> {
    typeface
        .with_clone_ok(|typeface| OwnedPtr::new(Font::from_typeface(typeface, size)))
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_font_is_force_auto_hinting(font: BorrowedPtr<Font>) -> bool {
    font.with_ref_ok(|font| font.is_force_auto_hinting())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_is_embedded_bitmaps(font: BorrowedPtr<Font>) -> bool {
    font.with_ref_ok(|font| font.is_embedded_bitmaps())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_is_subpixel(font: BorrowedPtr<Font>) -> bool {
    font.with_ref_ok(|font| font.is_subpixel()).or_log(false)
}

#[no_mangle]
pub fn skia_font_set_subpixel(mut font: BorrowedPtr<Font>, is_subpixel: bool) {
    font.with_mut_ok(|font| {
        font.set_subpixel(is_subpixel);
    })
    .log();
}

#[no_mangle]
pub fn skia_font_is_linear_metrics(font: BorrowedPtr<Font>) -> bool {
    font.with_ref_ok(|font| font.is_linear_metrics())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_is_embolden(font: BorrowedPtr<Font>) -> bool {
    font.with_ref_ok(|font| font.is_embolden()).or_log(false)
}

#[no_mangle]
pub fn skia_font_is_baseline_snap(font: BorrowedPtr<Font>) -> bool {
    font.with_ref_ok(|font| font.is_baseline_snap())
        .or_log(false)
}

#[no_mangle]
pub fn skia_font_get_edging(font: BorrowedPtr<Font>) -> Edging {
    font.with_ref_ok(|font| font.edging()).or_log(Edging::Alias)
}

#[no_mangle]
pub fn skia_font_set_edging(mut font: BorrowedPtr<Font>, font_edging: Edging) {
    font.with_mut_ok(|font| {
        font.set_edging(font_edging);
    })
    .log();
}

#[no_mangle]
pub fn skia_font_get_hinting(font: BorrowedPtr<Font>) -> FontHinting {
    font.with_ref_ok(|font| font.hinting())
        .or_log(FontHinting::None)
}

#[no_mangle]
pub fn skia_font_set_hinting(mut font: BorrowedPtr<Font>, font_hinting: FontHinting) {
    font.with_mut_ok(|font| {
        font.set_hinting(font_hinting);
    })
    .log();
}

#[no_mangle]
pub fn skia_font_get_typeface_or_default(font: BorrowedPtr<Font>) -> OwnedPtr<Typeface> {
    font.with_ref_ok(|font| OwnedPtr::new(font.typeface()))
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_font_get_size(font: BorrowedPtr<Font>) -> scalar {
    font.with_ref_ok(|font| font.size()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_scale_x(font: BorrowedPtr<Font>) -> scalar {
    font.with_ref_ok(|font| font.scale_x()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_skew_x(font: BorrowedPtr<Font>) -> scalar {
    font.with_ref_ok(|font| font.skew_x()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_spacing(font: BorrowedPtr<Font>) -> scalar {
    font.with_ref_ok(|font| font.spacing()).or_log(0.0)
}

#[no_mangle]
pub fn skia_font_get_metrics(font: BorrowedPtr<Font>) -> OwnedPtr<FontMetrics> {
    font.with_ref_ok(|font| OwnedPtr::new(font.metrics().1))
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_font_text_to_glyphs(
    font: BorrowedPtr<Font>,
    text_ptr: BorrowedPtr<StringBox>,
    _encoding: TextEncoding,
    mut glyphs_ptr: BorrowedPtr<ArrayBox<GlyphId>>,
    paint_ptr: BorrowedPtr<Paint>,
    mut bounds_ptr: BorrowedPtr<Rect>,
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
    font: BorrowedPtr<Font>,
    text_ptr: BorrowedPtr<StringBox>,
    _encoding: TextEncoding,
    paint_ptr: BorrowedPtr<Paint>,
    mut bounds_ptr: BorrowedPtr<Rect>,
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
pub fn skia_font_drop(mut ptr: OwnedPtr<Font>) {
    drop(ptr);
}
