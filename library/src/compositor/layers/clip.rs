use boxer::{ValueBox, ValueBoxPointer};
use compositor::compositor::CompositorContext;
use compositor::layers::layer::Layer;
use skia_safe::{scalar, Canvas, ClipOp, Path, RRect, Rect, Vector};
use std::cell::RefCell;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;

pub enum Clip {
    None,
    Rect(Rect),
    Path(Path),
    RRect(RRect),
}

impl Debug for Clip {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut formatter = f.debug_struct("Clip");

        let formatted = match self {
            Clip::Rect(rect) => {
                formatter.field("type", &String::from("Rect"));
                formatter.field("rect", rect)
            }
            Clip::Path(_) => formatter.field("type", &String::from("Path")),
            Clip::RRect(rrect) => {
                formatter.field("type", &String::from("RRect"));
                formatter.field("rrect", rrect.rect())
            }
            Clip::None => formatter.field("type", &String::from("None")),
        };
        formatted.finish()
    }
}

#[derive(Debug)]
pub struct ClipLayer {
    pub layers: Vec<Rc<RefCell<dyn Layer>>>,
    pub offset: Vector,
    pub clip: Clip,
}

impl ClipLayer {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            offset: Vector::new(0.0, 0.0),
            clip: Clip::None,
        }
    }

    pub fn rect(rect: Rect, offset: Vector) -> Self {
        let mut layer = Self::new();
        layer.set_offset(offset);
        layer.clip_rect(rect);
        layer
    }

    pub fn rrect(rrect: RRect, offset: Vector) -> Self {
        let mut layer = Self::new();
        layer.set_offset(offset);
        layer.clip_rrect(rrect);
        layer
    }

    pub fn path(path: Path, offset: Vector) -> Self {
        let mut layer = Self::new();
        layer.set_offset(offset);
        layer.clip_path(path);
        layer
    }

    pub fn set_offset(&mut self, offset: Vector) {
        self.offset = offset;
    }

    pub fn clip_rect(&mut self, rect: Rect) {
        self.clip = Clip::Rect(rect);
    }

    pub fn clip_rrect(&mut self, rrect: RRect) {
        self.clip = Clip::RRect(rrect);
    }

    pub fn clip_path(&mut self, path: Path) {
        self.clip = Clip::Path(path);
    }

    pub fn apply_on_canvas(&self, canvas: &mut Canvas) {
        match &self.clip {
            Clip::Rect(rect) => {
                canvas.clip_rect(rect.with_offset(self.offset), ClipOp::Intersect, true);
            }
            Clip::Path(path) => {
                canvas.clip_path(&path.with_offset(self.offset), ClipOp::Intersect, true);
            }
            Clip::RRect(rrect) => {
                canvas.clip_rrect(rrect.with_offset(self.offset), ClipOp::Intersect, true);
            }
            Clip::None => {}
        }
    }
}

impl Layer for ClipLayer {
    fn add_layer(&mut self, layer: Rc<RefCell<dyn Layer>>) {
        self.layers.push(layer);
    }

    fn count_layers(&self) -> usize {
        self.layers.len()
    }

    fn prepare(&mut self, context: &mut CompositorContext) {
        for layer in &self.layers {
            layer.borrow_mut().prepare(context);
        }
    }

    fn draw(&mut self, context: &mut CompositorContext) {
        let count = context.canvas().save();
        self.apply_on_canvas(context.canvas());
        for layer in &self.layers {
            layer.borrow_mut().draw(context);
        }
        context.canvas().restore_to_count(count);
    }
}

#[no_mangle]
pub fn skia_clip_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ClipLayer::new()));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_clip_layer_rect(
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ClipLayer::rect(
        Rect::new(left, top, right, bottom),
        Vector::new(offset_x, offset_y),
    )));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_clip_layer_rrect(
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
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(ClipLayer::rrect(
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
    )));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_clip_layer_path(
    path_ptr: *mut ValueBox<Path>,
    offset_x: scalar,
    offset_y: scalar,
) -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = path_ptr.with_value(
        || Rc::new(RefCell::new(ClipLayer::new())),
        |path| {
            Rc::new(RefCell::new(ClipLayer::path(
                path,
                Vector::new(offset_x, offset_y),
            )))
        },
    );
    ValueBox::new(layer).into_raw()
}
