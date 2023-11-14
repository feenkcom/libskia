use skia_safe::{FontStyle, FontStyleSet, Typeface};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_font_style_set_default() -> *mut ValueBox<FontStyleSet> {
    ValueBox::new(FontStyleSet::default()).into_raw()
}

#[no_mangle]
pub fn skia_font_style_get_count(font_style_set_ptr: *mut ValueBox<FontStyleSet>) -> usize {
    font_style_set_ptr.with_mut_ok(|set| set.count()).or_log(0)
}

#[no_mangle]
pub fn skia_font_style_set_count(font_style_set_ptr: *mut ValueBox<FontStyleSet>) -> usize {
    skia_font_style_get_count(font_style_set_ptr)
}

#[no_mangle]
pub fn skia_font_style_get_style_at(
    font_style_set_ptr: *mut ValueBox<FontStyleSet>,
    index: usize,
) -> *mut ValueBox<FontStyle> {
    font_style_set_ptr.with_mut_ok(|set| {
        ValueBox::new(set.style(index).0)
    }).into_raw()
}

#[no_mangle]
pub fn skia_font_style_set_style_at(
    font_style_set_ptr: *mut ValueBox<FontStyleSet>,
    index: usize,
) -> *mut ValueBox<FontStyle> {
    skia_font_style_get_style_at(font_style_set_ptr, index)
}

#[no_mangle]
pub fn skia_font_style_set_name_at(
    font_style_set_ptr: *mut ValueBox<FontStyleSet>,
    index: usize,
    _name_ptr: *mut ValueBox<StringBox>,
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
pub fn skia_font_style_set_drop(ptr: *mut ValueBox<FontStyleSet>) {
    ptr.release();
}
