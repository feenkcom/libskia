use skia_safe::textlayout::{Decoration, TextDecoration, TextDecorationMode, TextDecorationStyle};
use skia_safe::{Color, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_default() -> OwnedPtr<Decoration> {
    OwnedPtr::new(Decoration::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_color(
    mut decoration: BorrowedPtr<Decoration>,
    argb: u32,
) {
    decoration
        .with_mut_ok(|decoration| {
            decoration.color = Color::new(argb);
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_color(
    decoration: BorrowedPtr<Decoration>,
) -> OwnedPtr<Color> {
    decoration
        .with_clone_ok(|decoration| OwnedPtr::new(decoration.color))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_thickness(
    mut decoration: BorrowedPtr<Decoration>,
    thickness: scalar,
) {
    decoration
        .with_mut_ok(|decoration| {
            decoration.thickness_multiplier = thickness;
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_thickness(
    decoration: BorrowedPtr<Decoration>,
) -> scalar {
    decoration
        .with_clone_ok(|decoration| decoration.thickness_multiplier)
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_style(
    mut decoration: BorrowedPtr<Decoration>,
    style: TextDecorationStyle,
) {
    decoration
        .with_mut_ok(|decoration| {
            decoration.style = style;
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_style(
    decoration: BorrowedPtr<Decoration>,
) -> TextDecorationStyle {
    decoration
        .with_clone_ok(|decoration| decoration.style)
        .or_log(TextDecorationStyle::Solid)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_type(
    mut decoration: BorrowedPtr<Decoration>,
    ty: u32,
) {
    decoration
        .with_mut_ok(|decoration| {
            decoration.ty = TextDecoration::from_bits_retain(ty);
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_type(decoration: BorrowedPtr<Decoration>) -> u32 {
    decoration
        .with_clone_ok(|decoration| decoration.ty.bits())
        .or_log(TextDecoration::NO_DECORATION.bits())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_set_mode(
    mut decoration: BorrowedPtr<Decoration>,
    mode: TextDecorationMode,
) {
    decoration
        .with_mut_ok(|decoration| {
            decoration.mode = mode;
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_get_mode(
    decoration: BorrowedPtr<Decoration>,
) -> TextDecorationMode {
    decoration
        .with_clone_ok(|decoration| decoration.mode)
        .or_log(TextDecorationMode::Gaps)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_paragraph_decoration_drop(decoration: OwnedPtr<Decoration>) {
    drop(decoration);
}
