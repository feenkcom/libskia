use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::{FontStyle, Typeface};

#[no_mangle]
pub fn skia_typeface_default() -> *mut ValueBox<Typeface> {
    ValueBox::new(Typeface::default()).into_raw()
}

#[no_mangle]
pub fn skia_typeface_from_name(
    family_name_ptr: *mut ValueBox<BoxerString>,
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
pub fn skia_typeface_clone(typeface_ptr: *mut ValueBox<Typeface>) -> *mut ValueBox<Typeface> {
    typeface_ptr.with_not_null_value_return(std::ptr::null_mut(), |typeface| {
        ValueBox::new(typeface).into_raw()
    })
}

#[no_mangle]
pub fn skia_typeface_get_font_style(
    typeface_ptr: *mut ValueBox<Typeface>,
) -> *mut ValueBox<FontStyle> {
    ValueBox::new(typeface_ptr.with(
        || FontStyle::new(Weight::NORMAL, Width::NORMAL, Slant::Upright),
        |typeface| typeface.font_style(),
    ))
    .into_raw()
}

#[no_mangle]
pub fn skia_typeface_get_family_name(
    typeface_ptr: *mut ValueBox<Typeface>,
    _ptr_string: *mut ValueBox<BoxerString>,
) {
    typeface_ptr.with_not_null(|typeface| {
        _ptr_string.with_not_null(|string| string.set_string(typeface.family_name()))
    });
}

#[no_mangle]
pub fn skia_typeface_is_bold(typeface_ptr: *mut ValueBox<Typeface>) -> bool {
    typeface_ptr.with_not_null_return(false, |typeface| typeface.is_bold())
}

#[no_mangle]
pub fn skia_typeface_is_italic(typeface_ptr: *mut ValueBox<Typeface>) -> bool {
    typeface_ptr.with_not_null_return(false, |typeface| typeface.is_italic())
}

#[no_mangle]
pub fn skia_typeface_is_fixed_pitch(typeface_ptr: *mut ValueBox<Typeface>) -> bool {
    typeface_ptr.with_not_null_return(false, |typeface| typeface.is_fixed_pitch())
}

#[no_mangle]
pub fn skia_typeface_drop(ptr: &mut *mut ValueBox<Typeface>) {
    drop!(ptr);
}
