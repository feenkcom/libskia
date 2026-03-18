use skia_safe::textlayout::{Decoration, TextDecoration, TextDecorationMode, TextDecorationStyle};
use skia_safe::{Color, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_default() -> OwnedPtr<Decoration> {
    OwnedPtr::new(Decoration::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_color(mut ptr: BorrowedPtr<Decoration>, argb: u32) {
    ptr.with_mut_ok(|decoration| {
        decoration.color = Color::new(argb);
    })
    .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_color(ptr: BorrowedPtr<Decoration>) -> OwnedPtr<Color> {
    ptr.with_clone_ok(|decoration| OwnedPtr::new(decoration.color))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_thickness(
    mut ptr: BorrowedPtr<Decoration>,
    thickness: scalar,
) {
    ptr.with_mut_ok(|decoration| {
        decoration.thickness_multiplier = thickness;
    })
    .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_thickness(ptr: BorrowedPtr<Decoration>) -> scalar {
    ptr.with_clone_ok(|decoration| decoration.thickness_multiplier)
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_style(
    mut ptr: BorrowedPtr<Decoration>,
    style: TextDecorationStyle,
) {
    ptr.with_mut_ok(|decoration| {
        decoration.style = style;
    })
    .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_style(ptr: BorrowedPtr<Decoration>) -> TextDecorationStyle {
    ptr.with_clone_ok(|decoration| decoration.style)
        .or_log(TextDecorationStyle::Solid)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_type(mut ptr: BorrowedPtr<Decoration>, ty: u32) {
    ptr.with_mut_ok(|decoration| {
        decoration.ty = TextDecoration::from_bits_retain(ty);
    })
    .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_type(ptr: BorrowedPtr<Decoration>) -> u32 {
    ptr.with_clone_ok(|decoration| decoration.ty.bits())
        .or_log(TextDecoration::NO_DECORATION.bits())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_mode(
    mut ptr: BorrowedPtr<Decoration>,
    mode: TextDecorationMode,
) {
    ptr.with_mut_ok(|decoration| {
        decoration.mode = mode;
    })
    .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_mode(ptr: BorrowedPtr<Decoration>) -> TextDecorationMode {
    ptr.with_clone_ok(|decoration| decoration.mode)
        .or_log(TextDecorationMode::Gaps)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_drop(ptr: OwnedPtr<Decoration>) {
    drop(ptr);
}
