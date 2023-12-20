use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::{FontStyle, Typeface};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_typeface_default() -> *mut ValueBox<Typeface> {
    ValueBox::new(Typeface::default()).into_raw()
}

#[no_mangle]
pub fn skia_typeface_from_name(
    family_name_ptr: *mut ValueBox<StringBox>,
    font_style_ptr: *mut ValueBox<FontStyle>,
) -> *mut ValueBox<Typeface> {
    family_name_ptr.with_not_null_return(std::ptr::null_mut(), |family_name| {
        font_style_ptr.with_not_null_value_return(std::ptr::null_mut(), |font_style| {
            match Typeface::from_name(family_name.to_string(), font_style) {
                None => std::ptr::null_mut(),
                Some(typeface) => ValueBox::new(typeface).into_raw(),
            }
        })
    })
}

#[no_mangle]
pub fn skia_typeface_clone(typeface: *mut ValueBox<Typeface>) -> *mut ValueBox<Typeface> {
    typeface
        .with_clone_ok(|typeface| value_box!(typeface))
        .into_raw()
}

#[no_mangle]
pub fn skia_typeface_get_font_style(typeface: *mut ValueBox<Typeface>) -> *mut ValueBox<FontStyle> {
    typeface
        .with_mut_ok(|typeface| value_box!(typeface.font_style()))
        .or_else(|_| {
            Ok(value_box!(FontStyle::new(
                Weight::NORMAL,
                Width::NORMAL,
                Slant::Upright,
            )))
        })
        .into_raw()
}

#[no_mangle]
pub fn skia_typeface_get_family_name(
    typeface_ptr: *mut ValueBox<Typeface>,
    _ptr_string: *mut ValueBox<StringBox>,
) {
    typeface_ptr.with_not_null(
        |typeface| _ptr_string.with_not_null(|string| string.set_string(typeface.family_name()))
    );
}

#[no_mangle]
pub fn skia_typeface_is_bold(typeface_ptr: *mut ValueBox<Typeface>) -> bool {
    typeface_ptr
        .with_ref_ok(|typeface| typeface.is_bold())
        .or_log(false)
}

#[no_mangle]
pub fn skia_typeface_is_italic(typeface_ptr: *mut ValueBox<Typeface>) -> bool {
    typeface_ptr
        .with_ref_ok(|typeface| typeface.is_italic())
        .or_log(false)
}

#[no_mangle]
pub fn skia_typeface_is_fixed_pitch(typeface_ptr: *mut ValueBox<Typeface>) -> bool {
    typeface_ptr
        .with_ref_ok(|typeface| typeface.is_fixed_pitch())
        .or_log(false)
}

#[no_mangle]
pub fn skia_typeface_drop(ptr: *mut ValueBox<Typeface>) {
    ptr.release();
}
