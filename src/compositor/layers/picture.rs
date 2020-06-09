use boxer::boxes::{ValueBox, ValueBoxPointer};
use compositor::compositor::CompositorContext;
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use skia_safe::{Canvas, Image, Picture, Point, Rect, RoundOut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;
use std::sync::Arc;

pub struct PictureLayer {
    pub picture: Arc<Picture>,
    pub picture_id: u32,
    pub needs_cache: bool,
}

impl Debug for PictureLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("PictureLayer")
            .field("id", &self.picture_id)
            .field("needs_cache", &self.needs_cache)
            .finish()
    }
}

impl PictureLayer {
    pub fn from_picture(picture: Picture) -> Self {
        let id = picture.unique_id();

        Self {
            picture: Arc::new(picture),
            picture_id: id,
            needs_cache: true,
        }
    }
}

impl Layer for PictureLayer {
    fn prepare(&mut self, context: &mut CompositorContext) {
        if !self.needs_cache {
            return;
        }
        context.add_picture_to_rasterize(self.picture.clone());
    }

    fn draw(&mut self, context: &mut CompositorContext) {
        let (image, canvas) = context.get_rasterized_picture_image_and_canvas(self.picture_id);
        match image {
            None => {
                context.canvas().draw_picture(&self.picture, None, None);
            }
            Some(image) => {
                let current_matrix = canvas.total_matrix();

                let device_bounds = PictureToRasterize::compute_device_bounds(
                    &self.picture.cull_rect(),
                    &current_matrix,
                );
                canvas.save();

                canvas.concat(current_matrix.invert().as_ref().unwrap());
                canvas.draw_image(
                    image,
                    Point::new(device_bounds.left as f32, device_bounds.top as f32),
                    None,
                );
                canvas.restore();
            }
        }
    }
}

#[no_mangle]
pub fn skia_picture_layer_new_picture(
    mut _picture_ptr: *mut ValueBox<Picture>,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    _picture_ptr.with_not_null_value_return_block(
        || std::ptr::null_mut(),
        |picture| {
            let layer: Rc<RefCell<dyn Layer>> =
                Rc::new(RefCell::new(PictureLayer::from_picture(picture)));
            ValueBox::new(layer).into_raw()
        },
    )
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
