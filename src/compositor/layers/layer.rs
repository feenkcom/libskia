use compositor::compositor::RasterizerContext;
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

    fn draw_on(&mut self, context: RasterizerContext, canvas: &mut Canvas);
    fn take_picture_to_rasterize(
        &mut self,
        context: RasterizerContext,
        pictures: &mut Vec<PictureToRasterize>,
    );
    fn put_picture_after_rasterization(&mut self, pictures: &mut HashMap<u32, Picture>);
    fn take_image_from_cache(&mut self, picture_cache: &mut ImageCache);
    fn put_image_in_cache(&mut self, picture_cache: &mut ImageCache);
}