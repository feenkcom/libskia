use boxer::array::BoxerArray;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::font::Edging;
use skia_safe::{
    scalar, Font, FontHinting, FontMetrics, GlyphId, Paint, Rect, TextEncoding, Typeface,
};

#[no_mangle]
pub fn skia_font_default() -> *mut ValueBox<Font> {
    ValueBox::new(Font::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_from_typeface(
    typeface_ptr: *mut ValueBox<Typeface>,
    size: scalar,
) -> *mut ValueBox<Font> {
    typeface_ptr.with_not_null_value_return(std::ptr::null_mut(), |typeface| {
        ValueBox::new(Font::from_typeface(typeface, size)).into_raw()
    })
}

#[no_mangle]
pub fn skia_font_is_force_auto_hinting(font_ptr: *mut ValueBox<Font>) -> bool {
    font_ptr.with_not_null_return(false, |font| font.is_force_auto_hinting())
}

#[no_mangle]
pub fn skia_font_is_embedded_bitmaps(font_ptr: *mut ValueBox<Font>) -> bool {
    font_ptr.with_not_null_return(false, |font| font.is_embedded_bitmaps())
}

#[no_mangle]
pub fn skia_font_is_subpixel(font_ptr: *mut ValueBox<Font>) -> bool {
    font_ptr.with_not_null_return(false, |font| font.is_subpixel())
}

#[no_mangle]
pub fn skia_font_set_subpixel(font_ptr: *mut ValueBox<Font>, is_subpixel: bool) {
    font_ptr.with_not_null(|font| {
        font.set_subpixel(is_subpixel);
    });
}

#[no_mangle]
pub fn skia_font_is_linear_metrics(font_ptr: *mut ValueBox<Font>) -> bool {
    font_ptr.with_not_null_return(false, |font| font.is_linear_metrics())
}

#[no_mangle]
pub fn skia_font_is_embolden(font_ptr: *mut ValueBox<Font>) -> bool {
    font_ptr.with_not_null_return(false, |font| font.is_embolden())
}

#[no_mangle]
pub fn skia_font_is_baseline_snap(font_ptr: *mut ValueBox<Font>) -> bool {
    font_ptr.with_not_null_return(false, |font| font.is_baseline_snap())
}

#[no_mangle]
pub fn skia_font_get_edging(font_ptr: *mut ValueBox<Font>) -> Edging {
    font_ptr.with_not_null_return(Edging::Alias, |font| font.edging())
}

#[no_mangle]
pub fn skia_font_set_edging(font_ptr: *mut ValueBox<Font>, font_edging: Edging) {
    font_ptr.with_not_null(|font| {
        font.set_edging(font_edging);
    });
}

#[no_mangle]
pub fn skia_font_get_hinting(font_ptr: *mut ValueBox<Font>) -> FontHinting {
    font_ptr.with_not_null_return(FontHinting::None, |font| font.hinting())
}

#[no_mangle]
pub fn skia_font_set_hinting(font_ptr: *mut ValueBox<Font>, font_hinting: FontHinting) {
    font_ptr.with_not_null(|font| {
        font.set_hinting(font_hinting);
    });
}

#[no_mangle]
pub fn skia_font_get_typeface_or_default(font_ptr: *mut ValueBox<Font>) -> *mut ValueBox<Typeface> {
    font_ptr.with_not_null_return(std::ptr::null_mut(), |font| {
        ValueBox::new(font.typeface_or_default()).into_raw()
    })
}

#[no_mangle]
pub fn skia_font_get_size(font_ptr: *mut ValueBox<Font>) -> scalar {
    font_ptr.with_not_null_return(0.0, |font| font.size())
}

#[no_mangle]
pub fn skia_font_get_scale_x(font_ptr: *mut ValueBox<Font>) -> scalar {
    font_ptr.with_not_null_return(0.0, |font| font.scale_x())
}

#[no_mangle]
pub fn skia_font_get_skew_x(font_ptr: *mut ValueBox<Font>) -> scalar {
    font_ptr.with_not_null_return(0.0, |font| font.skew_x())
}

#[no_mangle]
pub fn skia_font_get_spacing(font_ptr: *mut ValueBox<Font>) -> scalar {
    font_ptr.with_not_null_return(0.0, |font| font.spacing())
}

#[no_mangle]
pub fn skia_font_get_metrics(font_ptr: *mut ValueBox<Font>) -> *mut ValueBox<FontMetrics> {
    font_ptr.with_not_null_return(std::ptr::null_mut(), |font| {
        ValueBox::new(font.metrics().1).into_raw()
    })
}

#[no_mangle]
pub fn skia_font_text_to_glyphs(
    font_ptr: *mut ValueBox<Font>,
    text_ptr: *mut ValueBox<BoxerString>,
    encoding: TextEncoding,
    glyphs_ptr: *mut ValueBox<BoxerArray<GlyphId>>,
    paint_ptr: *mut ValueBox<Paint>,
    bounds_ptr: *mut ValueBox<Rect>,
) -> scalar {
    font_ptr.with_not_null_return(0.0, |font| {
        glyphs_ptr.with_not_null_return(0.0, |glyphs| {
            text_ptr.with_not_null_return(0.0, |text| {
                let mut advance = 0.0;
                let glyphs_vec = font.text_to_glyphs_vec(text.as_bytes(), encoding);
                if glyphs_vec.len() > 0 {
                    paint_ptr.with_not_null(|paint| {
                        bounds_ptr.with_not_null(|bounds| {
                            // this is faster than computing ourselves
                            let (text_advance, text_bounds) =
                                font.measure_text(text.as_bytes(), encoding, Some(paint));
                            advance = text_advance;
                            bounds.set_ltrb(
                                text_bounds.left,
                                text_bounds.top,
                                text_bounds.right,
                                text_bounds.bottom,
                            );
                        });
                    });
                    glyphs.set_vector(glyphs_vec)
                }
                advance
            })
        })
    })
}

#[no_mangle]
pub fn skia_font_measure_text(
    font_ptr: *mut ValueBox<Font>,
    text_ptr: *mut ValueBox<BoxerString>,
    encoding: TextEncoding,
    paint_ptr: *mut ValueBox<Paint>,
    bounds_ptr: *mut ValueBox<Rect>,
) -> scalar {
    font_ptr.with_not_null_return(0.0, |font| {
        text_ptr.with_not_null_return(0.0, |text| {
            paint_ptr.with_not_null_return(0.0, |paint| {
                bounds_ptr.with_not_null_return(0.0, |bounds| {
                    let metrics = font.measure_text(text.as_bytes(), encoding, Some(paint));
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
}

#[no_mangle]
pub fn skia_font_drop(ptr: *mut ValueBox<Font>) {
    ptr.release();
}
