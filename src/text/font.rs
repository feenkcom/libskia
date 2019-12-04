use boxer::array::BoxerArray;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::{BoxerString, BoxerStringPointer};
use skia_safe::{
    scalar, Font, FontEdging, FontHinting, FontMetrics, GlyphId, Paint, Rect, TextEncoding,
    Typeface,
};

#[no_mangle]
pub fn skia_font_default() -> *mut ValueBox<Font> {
    ValueBox::new(Font::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_from_typeface(
    mut _typeface_ptr: *mut ValueBox<Typeface>,
    size: scalar,
) -> *mut ValueBox<Font> {
    _typeface_ptr.with_value_consumed(|typeface| {
        ValueBox::new(Font::from_typeface(typeface, size)).into_raw()
    })
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
pub fn skia_font_text_to_glyphs(
    _ptr: *mut ValueBox<Font>,
    _text_ptr: *mut BoxerString,
    encoding: TextEncoding,
    _glyphs_ptr: *mut ValueBox<BoxerArray<GlyphId>>,
    _paint_ptr: *mut ValueBox<Paint>,
    _bounds_ptr: *mut ValueBox<Rect>,
) -> scalar {
    let mut advance: scalar = 0.0;
    _ptr.with(|font| {
        _text_ptr.with(|text| {
            _glyphs_ptr.with(|glyphs| {
                let glyphs_vec = font.text_to_glyphs_vec(text.to_slice_u8(), encoding);
                if glyphs_vec.len() > 0 {
                    _paint_ptr.with_not_null(|paint| {
                        _bounds_ptr.with_not_null(|bounds| {
                            let mut glyphs_width: Vec<scalar> =
                                vec![Default::default(); glyphs_vec.len()];
                            let mut glyphs_bounds: Vec<Rect> =
                                vec![Default::default(); glyphs_vec.len()];

                            font.get_widths_bounds(
                                glyphs_vec.as_slice(),
                                Some(glyphs_width.as_mut_slice()),
                                Some(glyphs_bounds.as_mut_slice()),
                                Some(paint),
                            );

                            advance = glyphs_width.iter().sum();

                            let mut width = advance - *glyphs_width.last().unwrap_or(&0.0);
                            width = width + glyphs_bounds.last().unwrap().right;

                            for rect in glyphs_bounds.iter() {
                                bounds.join(rect)
                            }

                            bounds.set_ltrb(
                                glyphs_bounds[0].left,
                                bounds.top,
                                width,
                                bounds.bottom,
                            );
                        });
                    });

                    glyphs.set_vector(glyphs_vec)
                }
            })
        })
    });
    advance
}

#[no_mangle]
pub fn skia_font_measure_text(
    _ptr: *mut ValueBox<Font>,
    _text_ptr: *mut BoxerString,
    encoding: TextEncoding,
    _paint_ptr: *mut ValueBox<Paint>,
    _bounds_ptr: *mut ValueBox<Rect>,
) -> scalar {
    _ptr.with(|font| {
        _text_ptr.with(|text| {
            _paint_ptr.with(|paint| {
                _bounds_ptr.with(|bounds| {
                    let metrics = font.measure_text(text.to_slice_u8(), encoding, Some(paint));
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
pub fn skia_font_drop(_ptr: *mut ValueBox<Font>) {
    _ptr.drop();
}
