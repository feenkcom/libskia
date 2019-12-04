use boxer::array::BoxerArrayU8;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::{BoxerString, BoxerStringPointer};
use skia_safe::font_style::{Slant, Weight};
use skia_safe::{FontMgr, FontStyle, FontStyleSet, Typeface};
use text::font_style::FontStyleWidth;

#[no_mangle]
pub fn skia_font_manager_default() -> *mut ValueBox<FontMgr> {
    ValueBox::new(FontMgr::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_manager_count_families(_font_manager_ptr: *mut ValueBox<FontMgr>) -> usize {
    _font_manager_ptr.with_not_null_return(0, |font_manager| font_manager.count_families())
}

#[no_mangle]
pub fn skia_font_manager_get_family_name_at(
    _font_manager_ptr: *mut ValueBox<FontMgr>,
    _name_ptr: *mut BoxerString,
    index: usize,
) {
    _font_manager_ptr.with(|font_manager| {
        _name_ptr.with(|name| {
            name.set_string(font_manager.family_name(index));
        })
    });
}

#[no_mangle]
pub fn skia_font_manager_new_typeface_from_data(
    _font_manager_ptr: *mut ValueBox<FontMgr>,
    _data_ptr: *mut ValueBox<BoxerArrayU8>,
) -> *mut ValueBox<Typeface> {
    _font_manager_ptr.with(|font_manager| {
        _data_ptr.with(
            |data| match font_manager.new_from_data(data.to_slice(), None) {
                None => std::ptr::null_mut(),
                Some(typeface) => ValueBox::new(typeface).into_raw(),
            },
        )
    })
}

#[no_mangle]
pub fn skia_font_manager_match_family(
    _font_manager_ptr: *mut ValueBox<FontMgr>,
    _name_ptr: *mut BoxerString,
) -> *mut ValueBox<FontStyleSet> {
    _font_manager_ptr.with(|font_manager| {
        _name_ptr.with(|name| ValueBox::new(font_manager.match_family(name.to_string())).into_raw())
    })
}

#[no_mangle]
pub fn skia_font_manager_match_family_style(
    _font_manager_ptr: *mut ValueBox<FontMgr>,
    _name_ptr: *mut BoxerString,
    weight: i32,
    width: FontStyleWidth,
    slant: Slant,
) -> *mut ValueBox<Typeface> {
    _font_manager_ptr.with(|font_manager| {
        _name_ptr.with(|name| {
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
pub fn skia_font_manager_drop(_ptr: *mut ValueBox<FontMgr>) {
    _ptr.drop()
}
