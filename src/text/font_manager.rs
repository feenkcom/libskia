use boxer::array::BoxerArrayU8;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::font_style::{Slant, Weight};
use skia_safe::{FontMgr, FontStyle, FontStyleSet, Typeface};
use text::font_style::FontStyleWidth;

#[no_mangle]
pub fn skia_font_manager_default() -> *mut ValueBox<FontMgr> {
    ValueBox::new(FontMgr::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_manager_count_families(font_manager_ptr: *mut ValueBox<FontMgr>) -> usize {
    font_manager_ptr.with_not_null_return(0, |font_manager| font_manager.count_families())
}

#[no_mangle]
pub fn skia_font_manager_get_family_name_at(
    font_manager_ptr: *mut ValueBox<FontMgr>,
    name_ptr: *mut ValueBox<BoxerString>,
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
    font_manager_ptr: *mut ValueBox<FontMgr>,
    data_ptr: *mut ValueBox<BoxerArrayU8>,
) -> *mut ValueBox<Typeface> {
    font_manager_ptr.with_not_null_return(std::ptr::null_mut(), |font_manager| {
        data_ptr.with_not_null_return(std::ptr::null_mut(), |data| {
            match font_manager.new_from_data(data.to_slice(), None) {
                None => std::ptr::null_mut(),
                Some(typeface) => ValueBox::new(typeface).into_raw(),
            }
        })
    })
}

#[no_mangle]
pub fn skia_font_manager_match_family(
    font_manager_ptr: *mut ValueBox<FontMgr>,
    name_ptr: *mut ValueBox<BoxerString>,
) -> *mut ValueBox<FontStyleSet> {
    font_manager_ptr.with_not_null_return(std::ptr::null_mut(), |font_manager| {
        name_ptr.with_not_null_return(std::ptr::null_mut(), |name| {
            ValueBox::new(font_manager.match_family(name.to_string())).into_raw()
        })
    })
}

#[no_mangle]
pub fn skia_font_manager_match_family_style(
    font_manager_ptr: *mut ValueBox<FontMgr>,
    name_ptr: *mut ValueBox<BoxerString>,
    weight: i32,
    width: FontStyleWidth,
    slant: Slant,
) -> *mut ValueBox<Typeface> {
    font_manager_ptr.with_not_null_return(std::ptr::null_mut(), |font_manager| {
        name_ptr.with_not_null_return(std::ptr::null_mut(), |name| {
            match font_manager.match_family_style(
                name.to_string(),
                FontStyle::new(Weight::from(weight), width.into(), slant),
            ) {
                None => std::ptr::null_mut(),
                Some(typeface) => ValueBox::new(typeface).into_raw(),
            }
        })
    })
}

#[no_mangle]
pub fn skia_font_manager_drop(mut ptr: *mut ValueBox<FontMgr>) {
    ptr.drop()
}
