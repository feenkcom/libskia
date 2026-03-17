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
use value_box::{BorrowedPtr, ReturnBoxerResult};

use crate::text::font_style::FontStyleWidth;

#[no_mangle]
pub fn skia_enums_paint_style_to_string(enum_value: Style, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_blend_mode_to_string(enum_value: BlendMode, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_clip_op_to_string(enum_value: ClipOp, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_point_mode_to_string(enum_value: PointMode, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_alpha_type_to_string(enum_value: AlphaType, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_color_type_to_string(enum_value: ColorType, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_pixel_geometry_to_string(
    enum_value: PixelGeometry,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_cap_style_to_string(enum_value: Cap, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_join_style_to_string(enum_value: Join, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_tile_mode_to_string(enum_value: TileMode, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_path_fill_type_to_string(
    enum_value: PathFillType,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_style_slant_to_string(
    enum_value: Slant,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_style_width_to_string(
    enum_value: FontStyleWidth,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_edging_to_string(enum_value: Edging, mut string: BorrowedPtr<StringBox>) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_font_hinting_to_string(
    enum_value: FontHinting,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_enums_text_encoding_to_string(
    enum_value: TextEncoding,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_rounded_rectangle_type_to_string(
    enum_value: RRectType,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_rounded_rectangle_corner_to_string(
    enum_value: RRectCorner,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}

#[no_mangle]
pub fn skia_placeholder_alignment_to_string(
    enum_value: PlaceholderAlignment,
    mut string: BorrowedPtr<StringBox>,
) {
    string
        .with_mut_ok(|string| string.set_string(format!("{:?}", enum_value)))
        .log();
}
