use crate::text::font_style::FontStyleWidth;
use crate::value_box_compat::*;
use array_box::ArrayBox;
use skia_safe::font_style::{Slant, Weight};
use skia_safe::{FontMgr, FontStyle, FontStyleSet, Typeface};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr};

#[no_mangle]
pub fn skia_font_manager_default() -> OwnedPtr<FontMgr> {
    OwnedPtr::new(FontMgr::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_manager_count_families(font_manager_ptr: BorrowedPtr<FontMgr>) -> usize {
    font_manager_ptr.with_not_null_return(0, |font_manager| font_manager.count_families())
}

#[no_mangle]
pub fn skia_font_manager_get_family_name_at(
    font_manager_ptr: BorrowedPtr<FontMgr>,
    name_ptr: BorrowedPtr<StringBox>,
    index: usize,
) {
    font_manager_ptr.with_not_null(|font_manager| {
        name_ptr.with_not_null(|name| {
            name.set_string(font_manager.family_name(index));
        })
    });
}

#[no_mangle]
pub fn skia_font_manager_new_typeface_from_data(
    font_manager_ptr: BorrowedPtr<FontMgr>,
    data_ptr: BorrowedPtr<ArrayBox<u8>>,
) -> OwnedPtr<Typeface> {
    font_manager_ptr.with_not_null_return(OwnedPtr::null(), |font_manager| {
        data_ptr.with_not_null_return(OwnedPtr::null(), |data| {
            match font_manager.new_from_data(data.to_slice(), None) {
                None => OwnedPtr::null(),
                Some(typeface) => OwnedPtr::new(typeface),
            }
        })
    })
}

#[no_mangle]
pub fn skia_font_manager_match_family(
    font_manager_ptr: BorrowedPtr<FontMgr>,
    name_ptr: BorrowedPtr<StringBox>,
) -> OwnedPtr<FontStyleSet> {
    font_manager_ptr.with_not_null_return(OwnedPtr::null(), |font_manager| {
        name_ptr.with_not_null_return(OwnedPtr::null(), |name| {
            OwnedPtr::new(font_manager.match_family(name.to_string()))
        })
    })
}

#[no_mangle]
pub fn skia_font_manager_match_family_style(
    font_manager_ptr: BorrowedPtr<FontMgr>,
    name_ptr: BorrowedPtr<StringBox>,
    weight: i32,
    width: FontStyleWidth,
    slant: Slant,
) -> OwnedPtr<Typeface> {
    font_manager_ptr.with_not_null_return(OwnedPtr::null(), |font_manager| {
        name_ptr.with_not_null_return(OwnedPtr::null(), |name| {
            match font_manager.match_family_style(
                name.to_string(),
                FontStyle::new(Weight::from(weight), width.into(), slant),
            ) {
                None => OwnedPtr::null(),
                Some(typeface) => OwnedPtr::new(typeface),
            }
        })
    })
}

#[no_mangle]
pub fn skia_font_manager_drop(mut ptr: OwnedPtr<FontMgr>) {
    ptr.release();
}
