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
pub extern "C" fn skia_font_manager_count_families(font_manager_ptr: BorrowedPtr<FontMgr>) -> usize {
    font_manager_ptr
        .with_clone_ok(|font_manager| font_manager.count_families())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_get_family_name_at(
    mut font_manager_ptr: BorrowedPtr<FontMgr>,
    mut name_ptr: BorrowedPtr<StringBox>,
    index: usize,
) {
    font_manager_ptr
        .with_mut_ok(|font_manager| {
            name_ptr.with_mut_ok(|name| {
                name.set_string(font_manager.family_name(index));
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_new_typeface_from_data(
    font_manager_ptr: BorrowedPtr<FontMgr>,
    data_ptr: BorrowedPtr<ArrayBox<u8>>,
) -> OwnedPtr<Typeface> {
    font_manager_ptr
        .with_clone(|font_manager| {
            data_ptr.with_ref_ok(
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
    font_manager_ptr: BorrowedPtr<FontMgr>,
    name_ptr: BorrowedPtr<StringBox>,
) -> OwnedPtr<FontStyleSet> {
    font_manager_ptr
        .with_clone(|font_manager| {
            name_ptr.with_ref_ok(|name| OwnedPtr::new(font_manager.match_family(name.to_string())))
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_manager_match_family_style(
    font_manager_ptr: BorrowedPtr<FontMgr>,
    name_ptr: BorrowedPtr<StringBox>,
    weight: i32,
    width: FontStyleWidth,
    slant: Slant,
) -> OwnedPtr<Typeface> {
    font_manager_ptr
        .with_clone(|font_manager| {
            name_ptr.with_ref_ok(|name| {
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
pub extern "C" fn skia_font_manager_drop(ptr: OwnedPtr<FontMgr>) {
    drop(ptr);
}
