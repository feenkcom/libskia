use boxer::boxes::{ValueBox, ValueBoxPointer};
use compositor::compositor::CompositorContext;
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use compositor::shadow_cache::Shadow;
use skia_safe::{
    scalar, Canvas, Color, Image, Matrix, Path, Picture, Point, RRect, Rect, RoundOut, Vector,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;
use std::sync::Arc;

pub struct ShadowLayer {
    pub layers: Vec<Rc<RefCell<dyn Layer>>>,
    pub color: Color,
    pub radius: (scalar, scalar),
    pub offset: Vector,
    path: Path,
    cached_shadow: Option<Shadow>,
}

impl ShadowLayer {
    pub fn new(color: Color, radius: (scalar, scalar), offset: Vector, path: Path) -> Self {
        Self {
            layers: vec![],
            color,
            radius,
            offset,
            path,
            cached_shadow: None,
        }
    }

    pub fn cached_shadow(&mut self) -> &Shadow {
        if self.cached_shadow.is_none() {
            self.cached_shadow = Some(Shadow::new(
                self.color,
                self.radius,
                self.offset,
                self.path.clone(),
            ));
        }
        self.cached_shadow.as_ref().unwrap()
    }

    pub fn set_path(&mut self, path: Path) {
        self.path = path;
        self.cached_shadow = None;
    }
}

impl Debug for ShadowLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("ShadowLayer")
            .field("color", &self.color)
            .field("radius", &self.radius)
            .field("offset", &self.offset)
            .field("layers", &self.layers)
            .finish()
    }
}

impl Layer for ShadowLayer {
    fn add_layer(&mut self, layer: Rc<RefCell<dyn Layer>>) {
        self.layers.push(layer);
    }

    fn count_layers(&self) -> usize {
        self.layers.len()
    }

    fn prepare(&mut self, context: &mut CompositorContext) {
        context.add_shadow_to_rasterize(self.cached_shadow().clone());
        for layer in &self.layers {
            layer.borrow_mut().prepare(context);
        }
    }

    fn draw(&mut self, context: &mut CompositorContext) {
        let (image, canvas) = context.get_rasterized_shadow_image_and_canvas(self.cached_shadow());
        match image {
            None => {}
            Some((image, matrix)) => {
                let current_matrix = canvas.total_matrix();

                let device_bounds = PictureToRasterize::compute_device_bounds_rect(
                    &self.cached_shadow().cull_rect(),
                    &current_matrix,
                );

                canvas.save();

                let relative_matrix =
                    Matrix::concat(&current_matrix, matrix.invert().as_ref().unwrap());

                let relative_bounds = PictureToRasterize::compute_device_bounds(
                    &device_bounds.into(),
                    &relative_matrix.invert().unwrap(),
                );

                canvas.reset_matrix();
                canvas.set_matrix(&relative_matrix);
                canvas.draw_image(
                    image,
                    Point::new(relative_bounds.left as f32, relative_bounds.top as f32),
                    None,
                );
                canvas.restore();
            }
        }

        for layer in &self.layers {
            layer.borrow_mut().draw(context);
        }
    }
}

#[no_mangle]
pub fn skia_shadow_layer_new(
    delta_x: scalar,
    delta_y: scalar,
    sigma_x: scalar,
    sigma_y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ShadowLayer::new(
        Color::from_argb(a, r, g, b),
        (sigma_x, sigma_y),
        Vector::new(delta_x, delta_y),
        Path::new(),
    )));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_shadow_layer_set_path(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    _path_ptr: *mut ValueBox<Path>,
) {
    _ptr.with_not_null_value(|layer| {
        _path_ptr.with_not_null_value(|path| layer.borrow_mut().set_path(path))
    });
}

#[no_mangle]
pub fn skia_shadow_layer_set_rectangle(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        let mut path = Path::new();
        path.add_rect(Rect::new(left, top, right, bottom), None);
        layer.borrow_mut().set_path(path)
    })
}

#[no_mangle]
pub fn skia_shadow_layer_set_rounded_rectangle(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_left_x: scalar,
    r_left_y: scalar,
    r_top_x: scalar,
    r_top_y: scalar,
    r_right_x: scalar,
    r_right_y: scalar,
    r_bottom_x: scalar,
    r_bottom_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        let mut path = Path::new();
        let rect = Rect::new(left, top, right, bottom);
        let radii = [
            Vector::new(r_left_x, r_left_y),
            Vector::new(r_top_x, r_top_y),
            Vector::new(r_right_x, r_right_y),
            Vector::new(r_bottom_x, r_bottom_y),
        ];

        path.add_rrect(&RRect::new_rect_radii(rect, &radii), None);
        layer.borrow_mut().set_path(path)
    })
}

#[no_mangle]
pub fn skia_shadow_layer_set_circle(
    _ptr: *mut ValueBox<Rc<RefCell<ShadowLayer>>>,
    origin_x: scalar,
    origin_y: scalar,
    radius: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        let mut path = Path::new();
        path.add_circle(Point::new(origin_x, origin_y), radius, None);
        layer.borrow_mut().set_path(path)
    })
}
