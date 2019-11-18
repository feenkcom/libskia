use skia_safe::{FontStyleSet, FontStyle};
use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::{BoxerString, BoxerStringPointer};

#[no_mangle]
pub fn skia_font_style_set_default() -> *mut ValueBox<FontStyleSet> {
    ValueBox::new(FontStyleSet::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_style_set_count(_ptr: *mut ValueBox<FontStyleSet>) -> usize {
    _ptr.with(|set| set.count())
}

#[no_mangle]
pub fn skia_font_style_set_style_at(_ptr: *mut ValueBox<FontStyleSet>, index: usize) -> *mut ValueBox<FontStyle> {
    _ptr.with(|set| ValueBox::new(set.style(index).0).into_raw())
}

#[no_mangle]
pub fn skia_font_style_set_name_at(_ptr: *mut ValueBox<FontStyleSet>, index: usize, _name_ptr: *mut BoxerString) {
    _ptr.with(|set| {
        _name_ptr.with(|name| {
            name.set_string(match set.style(index).1 {
                None => { String::from("Hello, world!") },
                Some(string) => { string },
            }.parse().unwrap())
        })
    });
}

#[no_mangle]
pub fn skia_font_style_set_drop(_ptr: *mut ValueBox<FontStyleSet>) {
    _ptr.drop()
}