use compositor::compositor::RasterizerContext;
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use skia_safe::{Canvas, Picture, Point};
use std::collections::HashMap;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct OffsetLayer {
    pub layers: Vec<Rc<RefCell<dyn Layer>>>,
    pub offset: Point,
}

impl OffsetLayer {
    pub fn new(offset: Point) -> Self {
        Self {
            layers: vec![],
            offset,
        }
    }
}

impl Layer for OffsetLayer {
    fn add_layer(&mut self, layer: Rc<RefCell<dyn Layer>>) {
        self.layers.push(layer);
    }

    fn count_layers(&self) -> usize {
        self.layers.len()
    }

    fn draw_on(&mut self, context: RasterizerContext, canvas: &mut Canvas) {
        canvas.save();
        canvas.translate(self.offset);
        for layer in &self.layers {
            layer.borrow_mut().draw_on(context.translate(self.offset), canvas);
        }
        canvas.restore();
    }

    fn take_picture_to_rasterize(
        &mut self,
        context: RasterizerContext,
        mut pictures: &mut Vec<PictureToRasterize>,
    ) {
        for mut layer in &self.layers {
            layer.borrow_mut().take_picture_to_rasterize(context.translate(self.offset), pictures);
        }
    }

    fn put_picture_after_rasterization(&mut self, mut pictures: &mut HashMap<u32, Picture>) {
        for mut layer in &self.layers {
            layer.borrow_mut().put_picture_after_rasterization(pictures);
        }
    }

    fn take_image_from_cache(&mut self, picture_cache: &mut ImageCache) {
        for layer in &self.layers {
            layer.borrow_mut().take_image_from_cache(picture_cache);
        }
    }

    fn put_image_in_cache(&mut self, picture_cache: &mut ImageCache) {
        for mut layer in &self.layers {
            layer.borrow_mut().put_image_in_cache(picture_cache);
        }
    }
}
