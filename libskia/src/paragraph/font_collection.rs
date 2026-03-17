use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};
use skia_safe::FontMgr;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_font_collection_new() -> OwnedPtr<FontCollection> {
    OwnedPtr::new(FontCollection::new())
}

#[no_mangle]
pub fn skia_font_collection_font_managers_count(
    font_collection_ptr: BorrowedPtr<FontCollection>,
) -> usize {
    font_collection_ptr
        .with_ref_ok(|collection| collection.font_managers_count())
        .or_log(0)
}

#[no_mangle]
pub fn skia_font_collection_set_asset_font_manager(
    mut font_collection_ptr: BorrowedPtr<FontCollection>,
    typeface_font_provider: BorrowedPtr<TypefaceFontProvider>,
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
    mut font_collection_ptr: BorrowedPtr<FontCollection>,
    font_manager_ptr: BorrowedPtr<FontMgr>,
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
pub fn skia_font_collection_drop(mut ptr: OwnedPtr<FontCollection>) {
    drop(ptr);
}
