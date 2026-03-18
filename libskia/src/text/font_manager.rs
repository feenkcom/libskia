use crate::text::font_style::FontStyleWidth;
use array_box::ArrayBox;
use skia_safe::font_style::{Slant, Weight};
use skia_safe::{FontMgr, FontStyle, FontStyleSet, Typeface};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_default() -> OwnedPtr<FontMgr> {
    OwnedPtr::new(FontMgr::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_count_families(font_manager: BorrowedPtr<FontMgr>) -> usize {
    font_manager
        .with_clone_ok(|font_manager| font_manager.count_families())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_get_family_name_at(
    mut font_manager: BorrowedPtr<FontMgr>,
    mut name: BorrowedPtr<StringBox>,
    index: usize,
) {
    font_manager
        .with_mut_ok(|font_manager| {
            name.with_mut_ok(|name| {
                name.set_string(font_manager.family_name(index));
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_new_typeface_from_data(
    font_manager: BorrowedPtr<FontMgr>,
    data: BorrowedPtr<ArrayBox<u8>>,
) -> OwnedPtr<Typeface> {
    font_manager
        .with_clone(|font_manager| {
            data.with_ref_ok(
                |data| match font_manager.new_from_data(data.to_slice(), None) {
                    None => OwnedPtr::null(),
                    Some(typeface) => OwnedPtr::new(typeface),
                },
            )
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_match_family(
    font_manager: BorrowedPtr<FontMgr>,
    name: BorrowedPtr<StringBox>,
) -> OwnedPtr<FontStyleSet> {
    font_manager
        .with_clone(|font_manager| {
            name.with_ref_ok(|name| OwnedPtr::new(font_manager.match_family(name.to_string())))
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_match_family_style(
    font_manager: BorrowedPtr<FontMgr>,
    name: BorrowedPtr<StringBox>,
    weight: i32,
    width: FontStyleWidth,
    slant: Slant,
) -> OwnedPtr<Typeface> {
    font_manager
        .with_clone(|font_manager| {
            name.with_ref_ok(|name| {
                match font_manager.match_family_style(
                    name.to_string(),
                    FontStyle::new(Weight::from(weight), width.into(), slant),
                ) {
                    None => OwnedPtr::null(),
                    Some(typeface) => OwnedPtr::new(typeface),
                }
            })
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_drop(font_manager: OwnedPtr<FontMgr>) {
    drop(font_manager);
}
