use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::{FontStyle, FontStyleSet, Typeface};

#[no_mangle]
pub fn skia_font_style_set_default() -> *mut ValueBox<FontStyleSet> {
    ValueBox::new(FontStyleSet::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_style_set_count(font_style_set_ptr: *mut ValueBox<FontStyleSet>) -> usize {
    font_style_set_ptr.with_not_null_return(0, |set| set.count())
}

#[no_mangle]
pub fn skia_font_style_set_style_at(
    font_style_set_ptr: *mut ValueBox<FontStyleSet>,
    index: usize,
) -> *mut ValueBox<FontStyle> {
    font_style_set_ptr.with_not_null_return(std::ptr::null_mut(), |set| {
        ValueBox::new(set.style(index).0).into_raw()
    })
}

#[no_mangle]
pub fn skia_font_style_set_name_at(
    font_style_set_ptr: *mut ValueBox<FontStyleSet>,
    index: usize,
    _name_ptr: *mut ValueBox<BoxerString>,
) {
    font_style_set_ptr.with_not_null(|set| {
        _name_ptr.with_not_null(|name| {
            name.set_string(
                match set.style(index).1 {
                    None => String::from(""),
                    Some(string) => string,
                }
                .parse()
                .unwrap(),
            )
        })
    });
}

#[no_mangle]
pub fn skia_font_style_set_new_typeface(
    font_style_set_ptr: *mut ValueBox<FontStyleSet>,
    index: usize,
) -> *mut ValueBox<Typeface> {
    font_style_set_ptr.with_not_null_return(std::ptr::null_mut(), |set| {
        match set.new_typeface(index) {
            None => std::ptr::null_mut(),
            Some(typeface) => ValueBox::new(typeface).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_font_style_set_drop(mut ptr: *mut ValueBox<FontStyleSet>) {
    ptr.drop()
}
