use skia_safe::textlayout::{Decoration, TextDecoration, TextDecorationMode, TextDecorationStyle};
use skia_safe::{scalar, Color};
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_paragraph_decoration_default() -> *mut ValueBox<Decoration> {
    ValueBox::new(Decoration::default()).into_raw()
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_color(ptr: *mut ValueBox<Decoration>, argb: u32) {
    ptr.with_not_null(|decoration| {
        decoration.color = Color::new(argb);
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_color(ptr: *mut ValueBox<Decoration>) -> *mut ValueBox<Color> {
    ptr.with_not_null_return(std::ptr::null_mut(), |decoration| {
        ValueBox::new(decoration.color).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_thickness(ptr: *mut ValueBox<Decoration>, thickness: scalar) {
    ptr.with_not_null(|decoration| {
        decoration.thickness_multiplier = thickness;
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_thickness(ptr: *mut ValueBox<Decoration>) -> scalar {
    ptr.with_not_null_return(0.0, |decoration| decoration.thickness_multiplier)
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_style(
    ptr: *mut ValueBox<Decoration>,
    style: TextDecorationStyle,
) {
    ptr.with_not_null(|decoration| {
        decoration.style = style;
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_style(ptr: *mut ValueBox<Decoration>) -> TextDecorationStyle {
    ptr.with_not_null_return(TextDecorationStyle::Solid, |decoration| decoration.style)
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_type(ptr: *mut ValueBox<Decoration>, ty: u32) {
    ptr.with_not_null(|decoration| {
        decoration.ty = unsafe { TextDecoration::from_bits_retain(ty) };
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_type(ptr: *mut ValueBox<Decoration>) -> u32 {
    ptr.with_not_null_return(TextDecoration::NO_DECORATION.bits(), |decoration| {
        decoration.ty.bits()
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_set_mode(
    ptr: *mut ValueBox<Decoration>,
    mode: TextDecorationMode,
) {
    ptr.with_not_null(|decoration| {
        decoration.mode = mode;
    })
}

#[no_mangle]
pub fn skia_paragraph_decoration_get_mode(ptr: *mut ValueBox<Decoration>) -> TextDecorationMode {
    ptr.with_not_null_return(TextDecorationMode::Gaps, |decoration| decoration.mode)
}

#[no_mangle]
pub fn skia_paragraph_decoration_drop(ptr: *mut ValueBox<Decoration>) {
    ptr.release();
}
