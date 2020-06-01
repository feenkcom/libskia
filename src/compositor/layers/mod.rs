pub mod clip;
pub mod layer;
pub mod leftover_state;
pub mod offset;
pub mod picture;
pub mod transformation;

use boxer::boxes::{ValueBox, ValueBoxPointer};
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::layers::leftover_state::LeftoverStateLayer;
use compositor::layers::offset::OffsetLayer;
use compositor::layers::picture::PictureLayer;
use compositor::layers::transformation::TransformationLayer;
use skia_safe::image_filters::image;
use skia_safe::{
    scalar, Canvas, Image, Matrix, Path, Picture, Point, RRect, Rect, RoundOut, Vector,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[no_mangle]
pub fn skia_picture_layer_new_picture(
    mut _picture_ptr: *mut ValueBox<Picture>,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = _picture_ptr.with_not_null_value_return_block(
        || Rc::new(RefCell::new(PictureLayer::new())),
        |picture| Rc::new(RefCell::new(PictureLayer::from_picture(picture))),
    );
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_picture_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(PictureLayer::new()));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_picture_layer_get_needs_cache(_ptr: *mut ValueBox<Rc<RefCell<PictureLayer>>>) -> bool {
    _ptr.with_not_null_value_return_block(|| false, |layer| layer.borrow().needs_cache)
}

#[no_mangle]
pub fn skia_picture_layer_set_needs_cache(
    _ptr: *mut ValueBox<Rc<RefCell<PictureLayer>>>,
    needs_cache: bool,
) {
    _ptr.with_not_null_value(|layer| layer.borrow_mut().needs_cache = needs_cache);
}

#[no_mangle]
pub fn skia_offset_layer_new_point(x: scalar, y: scalar) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(OffsetLayer::new(Point::new(x, y))));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_offset_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> =
        Rc::new(RefCell::new(OffsetLayer::new(Point::new(0.0, 0.0))));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_offset_layer_get_x(_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>) -> scalar {
    _ptr.with_not_null_value_return_block(|| 0.0, |layer| layer.borrow().offset.x)
}

#[no_mangle]
pub fn skia_offset_layer_get_y(_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>) -> scalar {
    _ptr.with_not_null_value_return_block(|| 0.0, |layer| layer.borrow().offset.y)
}

#[no_mangle]
pub fn skia_offset_layer_set_offset(
    _ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>,
    x: scalar,
    y: scalar,
) {
    _ptr.with_not_null_value(|layer| layer.borrow_mut().offset = Point::new(x, y));
}

#[no_mangle]
pub fn skia_transformation_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(TransformationLayer::new(
        Matrix::new_identity(),
    )));
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
pub fn skia_leftover_state_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(LeftoverStateLayer::new()));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_rect(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().clip_rect(
            Rect::new(left, top, right, bottom),
            Vector::new(offset_x, offset_y),
        );
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_rrect(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_top_left: scalar,
    r_top_right: scalar,
    r_bottom_right: scalar,
    r_bottom_left: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().clip_rrect(
            RRect::new_rect_radii(
                Rect::new(left, top, right, bottom),
                &[
                    Vector::new(r_top_left, r_top_left),
                    Vector::new(r_top_right, r_top_right),
                    Vector::new(r_bottom_right, r_bottom_right),
                    Vector::new(r_bottom_left, r_bottom_left),
                ],
            ),
            Vector::new(offset_x, offset_y),
        );
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_path(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    _ptr_path: *mut ValueBox<Path>,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        _ptr_path.with_not_null_value(|path| {
            layer
                .borrow_mut()
                .clip_path(path, Vector::new(offset_x, offset_y));
        })
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_transform(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    scale_x: scalar,
    skew_x: scalar,
    trans_x: scalar,
    skew_y: scalar,
    scale_y: scalar,
    trans_y: scalar,
    persp_0: scalar,
    persp_1: scalar,
    persp_2: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().transform(
            Matrix::new_all(
                scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, persp_0, persp_1, persp_2,
            ),
            Vector::new(offset_x, offset_y),
        );
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
