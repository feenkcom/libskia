use skia_safe::canvas::PointMode;
use skia_safe::font::Edging;
use skia_safe::font_style::Slant;
use skia_safe::paint::{Cap, Join, Style};
use skia_safe::rrect::{Corner as RRectCorner, Type as RRectType};
use skia_safe::textlayout::PlaceholderAlignment;
use skia_safe::{
    AlphaType, BlendMode, ClipOp, ColorType, FontHinting, PathFillType, PixelGeometry,
    TextEncoding, TileMode,
};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use crate::text::font_style::FontStyleWidth;

#[no_mangle]
pub fn skia_enums_paint_style_to_string(enum_value: Style, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_blend_mode_to_string(enum_value: BlendMode, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_clip_op_to_string(enum_value: ClipOp, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_point_mode_to_string(enum_value: PointMode, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_alpha_type_to_string(enum_value: AlphaType, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_color_type_to_string(enum_value: ColorType, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_pixel_geometry_to_string(
    enum_value: PixelGeometry,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_cap_style_to_string(enum_value: Cap, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_join_style_to_string(enum_value: Join, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_tile_mode_to_string(enum_value: TileMode, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_path_fill_type_to_string(
    enum_value: PathFillType,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_style_slant_to_string(enum_value: Slant, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_style_width_to_string(
    enum_value: FontStyleWidth,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_edging_to_string(enum_value: Edging, string: *mut ValueBox<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_hinting_to_string(
    enum_value: FontHinting,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_text_encoding_to_string(
    enum_value: TextEncoding,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_rounded_rectangle_type_to_string(
    enum_value: RRectType,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_rounded_rectangle_corner_to_string(
    enum_value: RRectCorner,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_placeholder_alignment_to_string(
    enum_value: PlaceholderAlignment,
    string: *mut ValueBox<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}
