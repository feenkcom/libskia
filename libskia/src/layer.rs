use skia_safe::{scalar, Paint, Rect};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

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
    save_layer_ptr
        .with_mut_ok(|rec| {
            rec.bounds = Some(Rect::new(left, top, right, bottom));
        })
        .log();
}

#[no_mangle]
pub fn skia_layer_rec_set_paint(
    save_layer_ptr: *mut ValueBox<SaveLayerRecWrapper>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    save_layer_ptr
        .with_mut(|rec| paint_ptr.take_value().map(|paint| rec.paint = Some(paint)))
        .log();
}

#[no_mangle]
pub fn skia_layer_rec_drop(ptr: *mut ValueBox<SaveLayerRecWrapper>) {
    ptr.release();
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn default_layer() {
        let mut layer_ptr = skia_layer_rec_default();
        skia_layer_rec_drop(layer_ptr);
    }
}
