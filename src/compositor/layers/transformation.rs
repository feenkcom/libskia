use boxer::boxes::{ValueBox, ValueBoxPointer};
use compositor::compositor::CompositorContext;
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use skia_safe::{Canvas, Matrix, Picture, Point};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct TransformationLayer {
    pub layers: Vec<Rc<RefCell<dyn Layer>>>,
    pub matrix: Matrix,
}

impl TransformationLayer {
    pub fn new(matrix: Matrix) -> Self {
        Self {
            layers: vec![],
            matrix,
        }
    }
}

impl Layer for TransformationLayer {
    fn add_layer(&mut self, layer: Rc<RefCell<dyn Layer>>) {
        self.layers.push(layer);
    }

    fn count_layers(&self) -> usize {
        self.layers.len()
    }

    fn prepare(&mut self, context: &mut CompositorContext) {
        context.push_concat(&self.matrix);
        for layer in &self.layers {
            layer.borrow_mut().prepare(context);
        }
        context.pop_matrix();
    }

    fn draw(&mut self, context: &mut CompositorContext) {
        context.push_concat(&self.matrix);

        context.canvas().save();
        context.canvas().concat(&self.matrix);
        for layer in &self.layers {
            layer.borrow_mut().draw(context);
        }
        context.canvas().restore();

        context.pop_matrix();
    }
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
