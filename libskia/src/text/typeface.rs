use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::{FontMgr, FontStyle, Typeface};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_default() -> OwnedPtr<Typeface> {
    FontMgr::new()
        .legacy_make_typeface(None, FontStyle::normal())
        .map(OwnedPtr::new)
        .unwrap_or_else(OwnedPtr::null)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_from_name(
    family_name: BorrowedPtr<StringBox>,
    font_style: BorrowedPtr<FontStyle>,
) -> OwnedPtr<Typeface> {
    family_name
        .with_ref(|family_name| {
            font_style.with_clone_ok(|font_style| {
                FontMgr::new()
                    .legacy_make_typeface(Some(family_name.as_str()), font_style)
                    .map(|typeface| OwnedPtr::new(typeface))
            })
        })
        .or_log(None)
        .unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_clone(typeface: BorrowedPtr<Typeface>) -> OwnedPtr<Typeface> {
    typeface
        .with_clone_ok(|typeface| OwnedPtr::new(typeface))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_get_font_style(
    mut typeface: BorrowedPtr<Typeface>,
) -> OwnedPtr<FontStyle> {
    typeface
        .with_mut_ok(|typeface| OwnedPtr::new(typeface.font_style()))
        .or_log(OwnedPtr::new(FontStyle::new(
            Weight::NORMAL,
            Width::NORMAL,
            Slant::Upright,
        )))
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_get_family_name(
    mut typeface: BorrowedPtr<Typeface>,
    mut string: BorrowedPtr<StringBox>,
) {
    typeface
        .with_mut_ok(|typeface| {
            string.with_mut_ok(|string| string.set_string(typeface.family_name()))
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_is_bold(typeface: BorrowedPtr<Typeface>) -> bool {
    typeface
        .with_ref_ok(|typeface| typeface.is_bold())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_is_italic(typeface: BorrowedPtr<Typeface>) -> bool {
    typeface
        .with_ref_ok(|typeface| typeface.is_italic())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_is_fixed_pitch(typeface: BorrowedPtr<Typeface>) -> bool {
    typeface
        .with_ref_ok(|typeface| typeface.is_fixed_pitch())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_drop(typeface: OwnedPtr<Typeface>) {
    drop(typeface);
}
