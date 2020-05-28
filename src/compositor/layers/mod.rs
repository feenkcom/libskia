pub mod layer;
pub mod offset;
pub mod picture;
pub mod rectangle_clip;
pub mod transformation;

use boxer::boxes::{ValueBox, ValueBoxPointer};
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::layers::offset::OffsetLayer;
use compositor::layers::picture::PictureLayer;
use compositor::layers::transformation::TransformationLayer;
use skia_safe::image_filters::image;
use skia_safe::{scalar, Canvas, Image, Matrix, Picture, Point, RoundOut};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[no_mangle]
pub fn skia_picture_layer_new_picture(
    mut _picture_ptr: *mut ValueBox<Picture>,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = _picture_ptr.with_not_null_value_return_block(||{
        Rc::new(RefCell::new(PictureLayer::new()))
    },|picture| {
        Rc::new(RefCell::new(PictureLayer::from_picture(picture)))
    });
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_picture_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(PictureLayer::new()));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_offset_layer_new_point(x: scalar, y: scalar) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(OffsetLayer::new(Point::new(x, y))));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_offset_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(OffsetLayer::new(Point::new(0.0, 0.0))));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_offset_layer_get_x(_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>) -> scalar {
    _ptr.with_not_null_value_return_block(||{0.0}, |layer| layer.borrow().offset.x)
}

#[no_mangle]
pub fn skia_offset_layer_set_offset(_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>, x: scalar, y: scalar) {
    _ptr.with_not_null_value(|layer| { layer.borrow_mut().offset = Point::new(x, y)});
}

#[no_mangle]
pub fn skia_transformation_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(TransformationLayer::new(Matrix::new_identity())));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_transformation_layer_new_matrix(
    _ptr_matrix: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    _ptr_matrix.with_value(|matrix| {
        let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(TransformationLayer::new(matrix)));
        ValueBox::new(layer).into_raw()
    })
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
pub fn skia_composition_layer_count_layers(_layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>) -> usize {
    _layer_ptr.with_not_null_value_return_block(||{0}, |layer| layer.borrow().count_layers())
}

#[no_mangle]
pub fn skia_composition_layer_count_refs(_layer_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>) -> usize {
    _layer_ptr.with_not_null_value_return_block(||{0}, |layer| Rc::strong_count(&layer) - 1)
}

#[no_mangle]
pub fn skia_composition_layer_drop(_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>) {
    _ptr.drop();
}
