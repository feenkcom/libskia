use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::textlayout::FontCollection;
use skia_safe::FontMgr;

#[no_mangle]
pub fn skia_font_collection_new() -> *mut ValueBox<FontCollection> {
    ValueBox::new(FontCollection::new()).into_raw()
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
pub fn skia_font_collection_drop(ptr: &mut *mut ValueBox<FontCollection>) {
    drop!(ptr);
}
