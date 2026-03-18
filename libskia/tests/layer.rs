use Skia::layer::{skia_layer_rec_default, skia_layer_rec_drop};

#[test]
fn default_layer() {
    let layer = skia_layer_rec_default();
    skia_layer_rec_drop(layer);
}
