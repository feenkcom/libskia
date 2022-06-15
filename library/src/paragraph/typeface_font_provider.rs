use boxer::{ReturnBoxerResult, ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::textlayout::TypefaceFontProvider;
use skia_safe::Typeface;

#[no_mangle]
pub fn skia_typeface_font_provider_new() -> *mut ValueBox<TypefaceFontProvider> {
    ValueBox::new(TypefaceFontProvider::new()).into_raw()
}

#[no_mangle]
pub fn skia_typeface_font_provider_register_typeface(
    typeface_font_provider: *mut ValueBox<TypefaceFontProvider>,
    typeface: *mut ValueBox<Typeface>,
) -> usize {
    typeface_font_provider
        .to_ref()
        .and_then(|mut typeface_font_provider| {
            let alias: Option<&str> = None;
            typeface
                .to_ref()
                .map(|typeface| typeface_font_provider.register_typeface(typeface.clone(), alias))
        })
        .or_log(0)
}

#[no_mangle]
pub fn skia_typeface_font_provider_drop(ptr: &mut *mut ValueBox<TypefaceFontProvider>) {
    drop!(ptr);
}
