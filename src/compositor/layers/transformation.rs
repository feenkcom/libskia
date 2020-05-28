use compositor::compositor::RasterizerContext;
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use skia_safe::{Canvas, Matrix, Picture, Point};
use std::collections::HashMap;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use std::rc::Rc;
use std::cell::RefCell;

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

    fn draw_on(&mut self, context: RasterizerContext, canvas: &mut Canvas) {
        canvas.save();
        canvas.concat(&self.matrix);
        for layer in &self.layers {
            layer.borrow_mut().draw_on(context.concat(&self.matrix), canvas);
        }
        canvas.restore();
    }

    fn take_picture_to_rasterize(
        &mut self,
        context: RasterizerContext,
        mut pictures: &mut Vec<PictureToRasterize>,
    ) {
        for mut layer in &self.layers {
            layer.borrow_mut().take_picture_to_rasterize(context.concat(&self.matrix), pictures);
        }
    }

    fn put_picture_after_rasterization(&mut self, mut pictures: &mut HashMap<u32, Picture>) {
        for mut layer in &self.layers {
            layer.borrow_mut().put_picture_after_rasterization(pictures);
        }
    }

    fn take_image_from_cache(&mut self, picture_cache: &mut ImageCache) {
        for mut layer in &self.layers {
            layer.borrow_mut().take_image_from_cache(picture_cache);
        }
    }

    fn put_image_in_cache(&mut self, picture_cache: &mut ImageCache) {
        for mut layer in &self.layers {
            layer.borrow_mut().put_image_in_cache(picture_cache);
        }
    }
}
