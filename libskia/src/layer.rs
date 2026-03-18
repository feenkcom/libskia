use skia_safe::{Paint, Rect, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[derive(Default)]
#[repr(C)]
pub struct SaveLayerRecWrapper {
    pub bounds: Option<Rect>,
    pub paint: Option<Paint>,
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_layer_rec_default() -> OwnedPtr<SaveLayerRecWrapper> {
    OwnedPtr::new(SaveLayerRecWrapper::default())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_layer_rec_set_bounds(
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

#[unsafe(no_mangle)]
pub extern "C" fn skia_layer_rec_set_paint(
    mut save_layer_ptr: BorrowedPtr<SaveLayerRecWrapper>,
    paint_ptr: OwnedPtr<Paint>,
) {
    save_layer_ptr
        .with_mut(|rec| paint_ptr.with_value_ok(|paint| rec.paint = Some(paint)))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_layer_rec_drop(ptr: OwnedPtr<SaveLayerRecWrapper>) {
    drop(ptr);
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
