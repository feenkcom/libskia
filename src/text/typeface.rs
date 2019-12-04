use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::{BoxerString, BoxerStringPointer};
use skia_safe::{
    FontArguments, FontStyle, FontStyleSlant, FontStyleWeight, FontStyleWidth, Typeface,
};

#[no_mangle]
pub fn skia_typeface_default() -> *mut ValueBox<Typeface> {
    ValueBox::new(Typeface::default()).into_raw()
}

#[no_mangle]
pub fn skia_typeface_from_name(
    family_name_ptr: *mut BoxerString,
    font_style_ptr: *mut ValueBox<FontStyle>,
) -> *mut ValueBox<Typeface> {
    family_name_ptr.with(|family_name| {
        font_style_ptr.with_value(|font_style| {
            match Typeface::from_name(family_name.to_string(), font_style) {
                None => std::ptr::null_mut(),
                Some(typeface) => ValueBox::new(typeface).into_raw(),
            }
        })
    })
}

#[no_mangle]
pub fn skia_typeface_clone(_ptr: *mut ValueBox<Typeface>) -> *mut ValueBox<Typeface> {
    _ptr.with_not_null_return(std::ptr::null_mut(), |typeface| {
        match typeface.clone_with_arguments(&FontArguments::default()) {
            None => std::ptr::null_mut(),
            Some(typeface) => ValueBox::new(typeface).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_typeface_get_font_style(_ptr: *mut ValueBox<Typeface>) -> *mut ValueBox<FontStyle> {
    ValueBox::new(_ptr.with_not_null_return_block(
        || {
            FontStyle::new(
                FontStyleWeight::NORMAL,
                FontStyleWidth::NORMAL,
                FontStyleSlant::Upright,
            )
        },
        |typeface| typeface.font_style(),
    ))
    .into_raw()
}

#[no_mangle]
pub fn skia_typeface_get_family_name(_ptr: *mut ValueBox<Typeface>, _ptr_string: *mut BoxerString) {
    _ptr.with_not_null(|typeface| _ptr_string.with(|string| string.set_string(typeface.family_name())));
}

#[no_mangle]
pub fn skia_typeface_is_bold(_ptr: *mut ValueBox<Typeface>) -> bool {
    _ptr.with_not_null_return(false, |typeface| typeface.is_bold())
}

#[no_mangle]
pub fn skia_typeface_is_italic(_ptr: *mut ValueBox<Typeface>) -> bool {
    _ptr.with_not_null_return(false, |typeface| typeface.is_italic())
}

#[no_mangle]
pub fn skia_typeface_is_fixed_pitch(_ptr: *mut ValueBox<Typeface>) -> bool {
    _ptr.with_not_null_return(false,|typeface| typeface.is_fixed_pitch())
}

#[no_mangle]
pub fn skia_typeface_drop(_ptr: *mut ValueBox<Typeface>) {
    _ptr.drop();
}
