use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::{FontMgr, FontStyle, Typeface};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_typeface_default() -> OwnedPtr<Typeface> {
    FontMgr::new()
        .legacy_make_typeface(None, FontStyle::normal())
        .map(OwnedPtr::new)
        .unwrap_or_else(OwnedPtr::null)
}

#[no_mangle]
pub fn skia_typeface_from_name(
    family_name_ptr: BorrowedPtr<StringBox>,
    font_style_ptr: BorrowedPtr<FontStyle>,
) -> OwnedPtr<Typeface> {
    family_name_ptr
        .with_ref(|family_name| {
            font_style_ptr.with_clone_ok(|font_style| {
                FontMgr::new()
                    .legacy_make_typeface(Some(family_name.as_str()), font_style)
                    .map(|typeface| OwnedPtr::new(typeface))
            })
        })
        .or_log(None)
        .unwrap_or_default()
}

#[no_mangle]
pub fn skia_typeface_clone(typeface: BorrowedPtr<Typeface>) -> OwnedPtr<Typeface> {
    typeface
        .with_clone_ok(|typeface| OwnedPtr::new(typeface))
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_typeface_get_font_style(mut typeface: BorrowedPtr<Typeface>) -> OwnedPtr<FontStyle> {
    typeface
        .with_mut_ok(|typeface| OwnedPtr::new(typeface.font_style()))
        .or_log(OwnedPtr::new(FontStyle::new(
            Weight::NORMAL,
            Width::NORMAL,
            Slant::Upright,
        )))
}

#[no_mangle]
pub fn skia_typeface_get_family_name(
    mut typeface_ptr: BorrowedPtr<Typeface>,
    mut _ptr_string: BorrowedPtr<StringBox>,
) {
    typeface_ptr
        .with_mut_ok(|typeface| {
            _ptr_string.with_mut_ok(|string| string.set_string(typeface.family_name()))
        })
        .log();
}

#[no_mangle]
pub fn skia_typeface_is_bold(typeface_ptr: BorrowedPtr<Typeface>) -> bool {
    typeface_ptr
        .with_ref_ok(|typeface| typeface.is_bold())
        .or_log(false)
}

#[no_mangle]
pub fn skia_typeface_is_italic(typeface_ptr: BorrowedPtr<Typeface>) -> bool {
    typeface_ptr
        .with_ref_ok(|typeface| typeface.is_italic())
        .or_log(false)
}

#[no_mangle]
pub fn skia_typeface_is_fixed_pitch(typeface_ptr: BorrowedPtr<Typeface>) -> bool {
    typeface_ptr
        .with_ref_ok(|typeface| typeface.is_fixed_pitch())
        .or_log(false)
}

#[no_mangle]
pub fn skia_typeface_drop(ptr: OwnedPtr<Typeface>) {
    drop(ptr);
}
