use skia_safe::textlayout::TypefaceFontProvider;
use skia_safe::Typeface;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_typeface_font_provider_new() -> OwnedPtr<TypefaceFontProvider> {
    OwnedPtr::new(TypefaceFontProvider::new())
}

#[no_mangle]
pub fn skia_typeface_font_provider_register_typeface(
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

#[no_mangle]
pub fn skia_typeface_font_provider_drop(mut ptr: OwnedPtr<TypefaceFontProvider>) {
    drop(ptr);
}
