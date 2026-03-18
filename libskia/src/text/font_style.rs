use skia_safe::FontStyle;
use skia_safe::font_style::{Slant, Weight, Width};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_default() -> OwnedPtr<FontStyle> {
    OwnedPtr::new(FontStyle::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_new(
    weight: i32,
    width: FontStyleWidth,
    slant: Slant,
) -> OwnedPtr<FontStyle> {
    OwnedPtr::new(FontStyle::new(Weight::from(weight), width.into(), slant))
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_get_weight(font_style_ptr: BorrowedPtr<FontStyle>) -> i32 {
    font_style_ptr
        .with_clone_ok(|font_style| *font_style.weight())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_get_width(
    font_style_ptr: BorrowedPtr<FontStyle>,
) -> FontStyleWidth {
    font_style_ptr
        .with_clone_ok(|font_style| font_style.width().into())
        .or_log(FontStyleWidth::Normal)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_get_slant(font_style_ptr: BorrowedPtr<FontStyle>) -> Slant {
    font_style_ptr
        .with_clone_ok(|font_style| font_style.slant())
        .or_log(Slant::Upright)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_drop(ptr: OwnedPtr<FontStyle>) {
    drop(ptr);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum FontStyleWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl From<FontStyleWidth> for Width {
    fn from(width: FontStyleWidth) -> Width {
        match width {
            FontStyleWidth::UltraCondensed => Width::ULTRA_CONDENSED,
            FontStyleWidth::ExtraCondensed => Width::EXTRA_CONDENSED,
            FontStyleWidth::Condensed => Width::CONDENSED,
            FontStyleWidth::SemiCondensed => Width::SEMI_CONDENSED,
            FontStyleWidth::Normal => Width::NORMAL,
            FontStyleWidth::SemiExpanded => Width::SEMI_EXPANDED,
            FontStyleWidth::Expanded => Width::EXPANDED,
            FontStyleWidth::ExtraExpanded => Width::EXTRA_EXPANDED,
            FontStyleWidth::UltraExpanded => Width::ULTRA_EXPANDED,
        }
    }
}

impl From<Width> for FontStyleWidth {
    fn from(width: Width) -> FontStyleWidth {
        return if width == Width::ULTRA_CONDENSED {
            FontStyleWidth::UltraCondensed
        } else if width == Width::EXTRA_CONDENSED {
            FontStyleWidth::ExtraCondensed
        } else if width == Width::CONDENSED {
            FontStyleWidth::Condensed
        } else if width == Width::SEMI_CONDENSED {
            FontStyleWidth::SemiCondensed
        } else if width == Width::NORMAL {
            FontStyleWidth::Normal
        } else if width == Width::SEMI_EXPANDED {
            FontStyleWidth::SemiExpanded
        } else if width == Width::EXPANDED {
            FontStyleWidth::Expanded
        } else if width == Width::EXTRA_EXPANDED {
            FontStyleWidth::ExtraExpanded
        } else if width == Width::ULTRA_EXPANDED {
            FontStyleWidth::UltraExpanded
        } else {
            FontStyleWidth::Normal
        };
    }
}
