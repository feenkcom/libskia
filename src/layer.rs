use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::{scalar, Paint, Rect};

#[derive(Default)]
#[repr(C)]
pub struct SaveLayerRecWrapper {
    pub bounds: Option<Rect>,
    pub paint: Option<Paint>,
}

#[no_mangle]
pub fn skia_layer_rec_default() -> *mut ValueBox<SaveLayerRecWrapper> {
    ValueBox::new(SaveLayerRecWrapper::default()).into_raw()
}

#[no_mangle]
pub fn skia_layer_rec_set_bounds(
    save_layer_ptr: *mut ValueBox<SaveLayerRecWrapper>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) {
    save_layer_ptr.with_not_null(|rec| {
        rec.bounds = Some(Rect::new(left, top, right, bottom));
    });
}

#[no_mangle]
pub fn skia_layer_rec_set_paint(
    save_layer_ptr: *mut ValueBox<SaveLayerRecWrapper>,
    mut paint_ptr: *mut ValueBox<Paint>,
) {
    save_layer_ptr.with_not_null(|rec| {
        paint_ptr.with_not_null_value_consumed(|paint| rec.paint = Some(paint));
    });
}

#[no_mangle]
pub fn skia_layer_rec_drop(ptr: &mut *mut ValueBox<SaveLayerRecWrapper>) {
    drop!(ptr);
}

#[cfg(test)]
pub mod test {
    use layer::{skia_layer_rec_default, skia_layer_rec_drop};

    #[test]
    fn default_layer() {
        let layer_ptr = skia_layer_rec_default();

        skia_layer_rec_drop(&mut layer_ptr);
    }
}
