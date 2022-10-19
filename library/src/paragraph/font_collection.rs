use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};
use skia_safe::FontMgr;

#[no_mangle]
pub fn skia_font_collection_new() -> *mut ValueBox<FontCollection> {
    ValueBox::new(FontCollection::new()).into_raw()
}

#[no_mangle]
pub fn skia_font_collection_font_managers_count(
    font_collection_ptr: *mut ValueBox<FontCollection>,
) -> usize {
    font_collection_ptr.with_not_null_return(0, |collection| collection.font_managers_count())
}

#[no_mangle]
pub fn skia_font_collection_set_asset_font_manager(
    font_collection_ptr: *mut ValueBox<FontCollection>,
    typeface_font_provider: *mut ValueBox<TypefaceFontProvider>,
) {
    font_collection_ptr.with_not_null(|collection| {
        typeface_font_provider.with_not_null_value(|typeface_font_provider| {
            collection.set_asset_font_manager(Some(typeface_font_provider.into()));
        })
    })
}

#[no_mangle]
pub fn skia_font_collection_set_default_font_manager(
    font_collection_ptr: *mut ValueBox<FontCollection>,
    font_manager_ptr: *mut ValueBox<FontMgr>,
) {
    font_collection_ptr.with_not_null(|collection| {
        font_manager_ptr.with_not_null_value(|manager| {
            collection.set_default_font_manager(manager, None);
        })
    })
}

#[no_mangle]
pub fn skia_font_collection_drop(ptr: *mut ValueBox<FontCollection>) {
    ptr.release();
}
