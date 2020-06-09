use boxer::boxes::{ValueBox, ValueBoxPointer};
use compositor::compositor::CompositorContext;
use compositor::image_cache::ImageCache;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use skia_safe::{Canvas, Picture};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Layer: Debug {
    fn add_layer(&mut self, layer: Rc<RefCell<dyn Layer>>) {}
    fn count_layers(&self) -> usize {
        0
    }

    fn prepare(&mut self, context: &mut CompositorContext);
    fn draw(&mut self, context: &mut CompositorContext);
}

#[no_mangle]
pub fn skia_composition_layer_add(
    _target_layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
    mut _child_layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
) {
    _target_layer_ptr.with_not_null_value(|target_layer| {
        _child_layer_ptr.with_not_null_value(|child_layer| {
            target_layer.borrow_mut().add_layer(child_layer);
        })
    })
}

#[no_mangle]
pub fn skia_composition_layer_count_layers(
    _layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
) -> usize {
    _layer_ptr.with_not_null_value_return_block(|| 0, |layer| layer.borrow().count_layers())
}

#[no_mangle]
pub fn skia_composition_layer_count_refs(
    _layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
) -> usize {
    _layer_ptr.with_not_null_value_return_block(|| 0, |layer| Rc::strong_count(&layer) - 1)
}

#[no_mangle]
pub fn skia_composition_layer_drop(_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>) {
    _ptr.drop();
}
