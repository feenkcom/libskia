use crate::value_box_compat::*;
use skia_safe::textlayout::{Decoration, TextDecoration, TextDecorationMode, TextDecorationStyle};
use skia_safe::{scalar, Color};
use value_box::{BorrowedPtr, OwnedPtr};

#[no_mangle]
pub fn skia_paragraph_decoration_default() -> OwnedPtr<Decoration> {
    OwnedPtr::new(Decoration::default()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_color(ptr: BorrowedPtr<Decoration>, argb: u32) {
    ptr.with_not_null(|decoration| {
        decoration.color = Color::new(argb);
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_color(ptr: BorrowedPtr<Decoration>) -> OwnedPtr<Color> {
    ptr.with_not_null_return(OwnedPtr::null(), |decoration| {
        OwnedPtr::new(decoration.color)
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_thickness(ptr: BorrowedPtr<Decoration>, thickness: scalar) {
    ptr.with_not_null(|decoration| {
        decoration.thickness_multiplier = thickness;
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_thickness(ptr: BorrowedPtr<Decoration>) -> scalar {
    ptr.with_not_null_return(0.0, |decoration| decoration.thickness_multiplier)
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_style(
    ptr: BorrowedPtr<Decoration>,
    style: TextDecorationStyle,
) {
    ptr.with_not_null(|decoration| {
        decoration.style = style;
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_style(ptr: BorrowedPtr<Decoration>) -> TextDecorationStyle {
    ptr.with_not_null_return(TextDecorationStyle::Solid, |decoration| decoration.style)
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_type(ptr: BorrowedPtr<Decoration>, ty: u32) {
    ptr.with_not_null(|decoration| {
        decoration.ty = TextDecoration::from_bits_retain(ty);
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_type(ptr: BorrowedPtr<Decoration>) -> u32 {
    ptr.with_not_null_return(TextDecoration::NO_DECORATION.bits(), |decoration| {
        decoration.ty.bits()
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_mode(ptr: BorrowedPtr<Decoration>, mode: TextDecorationMode) {
    ptr.with_not_null(|decoration| {
        decoration.mode = mode;
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_mode(ptr: BorrowedPtr<Decoration>) -> TextDecorationMode {
    ptr.with_not_null_return(TextDecorationMode::Gaps, |decoration| decoration.mode)
}

#[no_mangle]
pub fn skia_paragraph_decoration_drop(mut ptr: OwnedPtr<Decoration>) {
    ptr.release();
}
