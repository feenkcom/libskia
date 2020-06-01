use compositor::compositor::RasterizerContext;
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use skia_safe::{Canvas, Image, Picture, Point, Rect, RoundOut};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

pub struct PictureLayer {
    pub picture: Option<Picture>,
    pub picture_id: u32,
    pub image: Option<Image>,
    pub needs_cache: bool,
}

impl Debug for PictureLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("PictureLayer")
            .field("id", &self.picture_id)
            .field("has_image", &self.image.is_some())
            .field("needs_cache", &self.needs_cache)
            .finish()
    }
}

impl PictureLayer {
    pub fn from_picture(picture: Picture) -> Self {
        let id = picture.unique_id();
        let logical_bounds = picture.cull_rect();

        Self {
            picture: Some(picture),
            picture_id: id,
            image: None,
            needs_cache: true,
        }
    }

    pub fn new() -> Self {
        Self {
            picture: None,
            picture_id: 0,
            image: None,
            needs_cache: true,
        }
    }
}

impl Layer for PictureLayer {
    fn draw_on(&mut self, context: RasterizerContext, canvas: &mut Canvas) {
        if self.image.is_some() {
            let device_bounds = PictureToRasterize::compute_device_bounds(
                &self.picture.as_ref().unwrap().cull_rect(),
                &canvas.total_matrix(),
            );
            canvas.save();
            canvas.concat(context.matrix.invert().as_ref().unwrap());
            canvas.draw_image(
                self.image.as_ref().unwrap(),
                Point::new(device_bounds.left as f32, device_bounds.top as f32),
                None,
            );
            canvas.restore();
            return;
        }

        match self.picture.as_ref() {
            None => {}
            Some(picture) => {
                canvas.draw_picture(picture, None, None);
            }
        }
    }
    fn take_picture_to_rasterize(
        &mut self,
        context: RasterizerContext,
        mut pictures: &mut Vec<PictureToRasterize>,
    ) {
        if self.needs_cache && self.image.is_none() {
            match self.picture.take() {
                None => {}
                Some(picture) => {
                    pictures.push(PictureToRasterize::new(picture, context.matrix));
                }
            }
        }
    }
    fn put_picture_after_rasterization(&mut self, mut pictures: &mut HashMap<u32, Picture>) {
        match pictures.remove(&self.picture_id) {
            None => {}
            Some(picture) => {
                self.picture = Some(picture);
            }
        }
    }

    fn take_image_from_cache(&mut self, mut picture_cache: &mut ImageCache) {
        match self.picture.as_ref() {
            None => {}
            Some(picture) => match picture_cache.pop_picture_image(picture) {
                None => {}
                Some(image) => self.image = Some(image),
            },
        }
    }
    fn put_image_in_cache(&mut self, mut picture_cache: &mut ImageCache) {
        match self.picture.as_ref() {
            None => {}
            Some(picture) => match self.image.take() {
                None => {}
                Some(image) => picture_cache.push_picture_image(picture, image),
            },
        }
    }
}
