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
    mut save_layer: BorrowedPtr<SaveLayerRecWrapper>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) {
    save_layer
        .with_mut_ok(|rec| {
            rec.bounds = Some(Rect::new(left, top, right, bottom));
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_layer_rec_set_paint(
    mut save_layer: BorrowedPtr<SaveLayerRecWrapper>,
    paint: OwnedPtr<Paint>,
) {
    save_layer
        .with_mut(|rec| paint.with_value_ok(|paint| rec.paint = Some(paint)))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_layer_rec_drop(save_layer_rec: OwnedPtr<SaveLayerRecWrapper>) {
    drop(save_layer_rec);
}
