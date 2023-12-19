use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};
use skia_safe::FontMgr;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_font_collection_new() -> *mut ValueBox<FontCollection> {
    ValueBox::new(FontCollection::new()).into_raw()
}

#[no_mangle]
pub fn skia_font_collection_font_managers_count(
    font_collection_ptr: *mut ValueBox<FontCollection>,
) -> usize {
    font_collection_ptr
        .with_ref_ok(|collection| collection.font_managers_count())
        .or_log(0)
}

#[no_mangle]
pub fn skia_font_collection_set_asset_font_manager(
    font_collection_ptr: *mut ValueBox<FontCollection>,
    typeface_font_provider: *mut ValueBox<TypefaceFontProvider>,
) {
    font_collection_ptr
        .with_mut(|font_collection| {
            typeface_font_provider.with_ref_ok(|typeface_font_provider| {
                font_collection.set_asset_font_manager(Some(Into::<FontMgr>::into(
                    typeface_font_provider.clone(),
                )));
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_font_collection_set_default_font_manager(
    font_collection_ptr: *mut ValueBox<FontCollection>,
    font_manager_ptr: *mut ValueBox<FontMgr>,
) {
    font_collection_ptr
        .with_mut(|font_collection| {
            font_manager_ptr.with_ref_ok(|font_manager| {
                font_collection.set_default_font_manager(Some(font_manager.clone()), None);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_font_collection_drop(ptr: *mut ValueBox<FontCollection>) {
    ptr.release();
}
