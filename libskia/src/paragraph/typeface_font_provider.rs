use skia_safe::Typeface;
use skia_safe::textlayout::TypefaceFontProvider;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_font_provider_new() -> OwnedPtr<TypefaceFontProvider> {
    OwnedPtr::new(TypefaceFontProvider::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_font_provider_register_typeface(
    mut typeface_font_provider: BorrowedPtr<TypefaceFontProvider>,
    typeface: BorrowedPtr<Typeface>,
) -> usize {
    typeface_font_provider
        .with_mut(|typeface_font_provider| {
            let alias: Option<&str> = None;
            typeface.with_ref_ok(|typeface| {
                typeface_font_provider.register_typeface(typeface.clone(), alias)
            })
        })
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_typeface_font_provider_drop(ptr: OwnedPtr<TypefaceFontProvider>) {
    drop(ptr);
}
