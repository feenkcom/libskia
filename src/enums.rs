use boxer::string::BoxerString;
use boxer::CBox;
use skia_safe::canvas::PointMode;
use skia_safe::font_style::Slant;
use skia_safe::paint::{Cap, Join, Style};
use skia_safe::{
    AlphaType, BlendMode, ClipOp, ColorType, FilterQuality, FontEdging, FontHinting, PathFillType,
    PixelGeometry, RRectCorner, RRectType, TextEncoding, TileMode,
};
use text::font_style::FontStyleWidth;

#[no_mangle]
pub fn skia_enums_filter_quality_to_string(_enum: FilterQuality, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_paint_style_to_string(_enum: Style, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_blend_mode_to_string(_enum: BlendMode, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_clip_op_to_string(_enum: ClipOp, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_point_mode_to_string(_enum: PointMode, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_alpha_type_to_string(_enum: AlphaType, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_color_type_to_string(_enum: ColorType, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_pixel_geometry_to_string(_enum: PixelGeometry, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_cap_style_to_string(_enum: Cap, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_join_style_to_string(_enum: Join, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_tile_mode_to_string(_enum: TileMode, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_path_fill_type_to_string(_enum: PathFillType, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_font_style_slant_to_string(_enum: Slant, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_font_style_width_to_string(_enum: FontStyleWidth, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_font_edging_to_string(_enum: FontEdging, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_font_hinting_to_string(_enum: FontHinting, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_enums_text_encoding_to_string(_enum: TextEncoding, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_rounded_rectangle_type_to_string(_enum: RRectType, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}

#[no_mangle]
pub fn skia_rounded_rectangle_corner_to_string(_enum: RRectCorner, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {}
        Some(string) => string.set_string(format!("{:?}", _enum)),
    })
}
