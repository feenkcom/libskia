use skia_safe::{FilterQuality, BlendMode, ClipOp, AlphaType, ColorType, PixelGeometry};
use boxer::string::BoxerString;
use boxer::CBox;
use skia_safe::paint::Style;
use skia_safe::canvas::PointMode;

#[no_mangle]
pub fn skia_enums_filter_quality_to_string(_enum: FilterQuality, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_enums_paint_style_to_string(_enum: Style, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_enums_blend_mode_to_string(_enum: BlendMode, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_enums_clip_op_to_string(_enum: ClipOp, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_enums_point_mode_to_string(_enum: PointMode, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_enums_alpha_type_to_string(_enum: AlphaType, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_enums_color_type_to_string(_enum: ColorType, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_enums_pixel_geometry_to_string(_enum: PixelGeometry, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}