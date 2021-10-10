use boxer::{ValueBox, ValueBoxPointer};
use compositor::compositor::CompositorContext;
use compositor::layers::layer::Layer;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use compositor::shadow_cache::Shadow;
use skia_safe::image_filters::drop_shadow_only;
use skia_safe::paint::Style;
use skia_safe::{scalar, BlendMode, Color, Matrix, Paint, Path, Point, RRect, Rect, Vector, M44};
use std::cell::RefCell;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;

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
            None => {
                let drop_shadow_filter =
                    drop_shadow_only(self.offset, self.radius, self.color, None, None);

                let mut shadow_paint = Paint::default();
                shadow_paint.set_style(Style::Stroke);
                shadow_paint.set_color(Color::WHITE);
                shadow_paint.set_anti_alias(true);
                shadow_paint.set_blend_mode(BlendMode::SrcOver);
                shadow_paint.set_stroke_width(if self.radius.0 > self.radius.1 {
                    self.radius.0
                } else {
                    self.radius.1
                });
                shadow_paint.set_image_filter(drop_shadow_filter);

                canvas.draw_path(&self.path, &shadow_paint);
            }
            Some((image, matrix)) => {
                let current_matrix = canvas.local_to_device_as_3x3();

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
                canvas.set_matrix(&M44::from(relative_matrix));
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
    r_left: scalar,
    r_top: scalar,
    r_right: scalar,
    r_bottom: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        let mut path = Path::new();
        let rect = Rect::new(left, top, right, bottom);
        let radii = [
            Vector::new(r_left, r_left),
            Vector::new(r_top, r_top),
            Vector::new(r_right, r_right),
            Vector::new(r_bottom, r_bottom),
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
