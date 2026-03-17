use crate::value_box_compat::*;
use skia_safe::{scalar, Paint, Rect};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[derive(Default)]
#[repr(C)]
pub struct SaveLayerRecWrapper {
    pub bounds: Option<Rect>,
    pub paint: Option<Paint>,
}

#[no_mangle]
pub fn skia_layer_rec_default() -> OwnedPtr<SaveLayerRecWrapper> {
    OwnedPtr::new(SaveLayerRecWrapper::default()).into_raw()
}

#[no_mangle]
pub fn skia_layer_rec_set_bounds(
    mut save_layer_ptr: BorrowedPtr<SaveLayerRecWrapper>,
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
    mut save_layer_ptr: BorrowedPtr<SaveLayerRecWrapper>,
    mut paint_ptr: OwnedPtr<Paint>,
) {
    save_layer_ptr
        .with_mut(|rec| paint_ptr.take_value().map(|paint| rec.paint = Some(paint)))
        .log();
}

#[no_mangle]
pub fn skia_layer_rec_drop(mut ptr: OwnedPtr<SaveLayerRecWrapper>) {
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
