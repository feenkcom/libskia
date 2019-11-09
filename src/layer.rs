use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{scalar, Rect, Paint};

#[derive(Default)]
#[repr(C)]
pub struct SaveLayerRecWrapper {
    pub bounds: Option<Rect>,
    pub paint: Option<Paint>
}

#[no_mangle]
pub fn skia_layer_rec_default() -> *mut ValueBox<SaveLayerRecWrapper> {
    ValueBox::new(SaveLayerRecWrapper::default()).into_raw()
}

#[no_mangle]
pub fn skia_layer_rec_set_bounds(mut _ptr: *mut ValueBox<SaveLayerRecWrapper>, left: scalar, top: scalar, right: scalar, bottom: scalar) {
    _ptr.with_not_null(|rec| {
        rec.bounds = Some(Rect::new(left, top, right, bottom));
    });
}

#[no_mangle]
pub fn skia_layer_rec_set_paint(mut _ptr: *mut ValueBox<SaveLayerRecWrapper>, mut _paint_ptr: *mut ValueBox<Paint>) {
    _ptr.with_not_null(|rec| {
        _paint_ptr.with_value_consumed(|paint| rec.paint = Some(paint));
    });
}

#[no_mangle]
pub fn skia_layer_rec_drop(_ptr: *mut ValueBox<SaveLayerRecWrapper>) {
    _ptr.drop()
}

#[cfg(test)]
pub mod test {
    use layer::{skia_layer_rec_default, skia_layer_rec_drop};

    #[test]
    fn default_layer() {
        let layer_ptr = skia_layer_rec_default();

        skia_layer_rec_drop(layer_ptr);
    }
}