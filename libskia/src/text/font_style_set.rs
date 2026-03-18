use skia_safe::{FontStyle, FontStyleSet, Typeface};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_set_default() -> OwnedPtr<FontStyleSet> {
    OwnedPtr::new(FontStyleSet::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_get_count(mut font_style_set_ptr: BorrowedPtr<FontStyleSet>) -> usize {
    font_style_set_ptr.with_mut_ok(|set| set.count()).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_set_count(font_style_set_ptr: BorrowedPtr<FontStyleSet>) -> usize {
    skia_font_style_get_count(font_style_set_ptr)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_get_style_at(
    mut font_style_set_ptr: BorrowedPtr<FontStyleSet>,
    index: usize,
) -> OwnedPtr<FontStyle> {
    font_style_set_ptr
        .with_mut_ok(|set| OwnedPtr::new(set.style(index).0))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_set_style_at(
    font_style_set_ptr: BorrowedPtr<FontStyleSet>,
    index: usize,
) -> OwnedPtr<FontStyle> {
    skia_font_style_get_style_at(font_style_set_ptr, index)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_set_name_at(
    mut font_style_set_ptr: BorrowedPtr<FontStyleSet>,
    index: usize,
    mut _name_ptr: BorrowedPtr<StringBox>,
) {
    font_style_set_ptr
        .with_mut_ok(|set| {
            _name_ptr.with_mut_ok(|name| {
                name.set_string(
                    match set.style(index).1 {
                        None => String::from(""),
                        Some(string) => string,
                    }
                    .parse()
                    .unwrap(),
                )
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_set_new_typeface(
    mut font_style_set_ptr: BorrowedPtr<FontStyleSet>,
    index: usize,
) -> OwnedPtr<Typeface> {
    font_style_set_ptr
        .with_mut_ok(|set| match set.new_typeface(index) {
            None => OwnedPtr::null(),
            Some(typeface) => OwnedPtr::new(typeface),
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_font_style_set_drop(ptr: OwnedPtr<FontStyleSet>) {
    drop(ptr);
}
