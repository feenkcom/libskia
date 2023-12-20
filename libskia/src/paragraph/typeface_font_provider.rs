use skia_safe::textlayout::TypefaceFontProvider;
use skia_safe::Typeface;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

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
        .with_mut(|typeface_font_provider| {
            let alias: Option<&str> = None;
            typeface.with_ref_ok(
                |typeface| typeface_font_provider.register_typeface(typeface.clone(), alias)
            )
        })
        .or_log(0)
}

#[no_mangle]
pub fn skia_typeface_font_provider_drop(ptr: *mut ValueBox<TypefaceFontProvider>) {
    ptr.release();
}
