use boxer::{ValueBox, ValueBoxPointer};
use compositor::compositor::CompositorContext;
use compositor::layers::layer::Layer;
use skia_safe::{scalar, Point};
use std::cell::RefCell;
use std::rc::Rc;

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

    fn prepare(&mut self, context: &mut CompositorContext) {
        context.push_translate(self.offset);
        for layer in &self.layers {
            layer.borrow_mut().prepare(context);
        }
        context.pop_matrix();
    }

    fn draw(&mut self, context: &mut CompositorContext) {
        context.push_translate(self.offset);

        context.canvas().save();
        context.canvas().translate(self.offset);
        for layer in &self.layers {
            layer.borrow_mut().draw(context);
        }
        context.canvas().restore();

        context.pop_matrix();
    }
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
pub fn skia_offset_layer_get_x(layer_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>) -> scalar {
    layer_ptr.with_not_null_value_return(0.0, |layer| layer.borrow().offset.x)
}

#[no_mangle]
pub fn skia_offset_layer_get_y(layer_ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>) -> scalar {
    layer_ptr.with_not_null_value_return(0.0, |layer| layer.borrow().offset.y)
}

#[no_mangle]
pub fn skia_offset_layer_set_offset(
    _ptr: *mut ValueBox<Rc<RefCell<OffsetLayer>>>,
    x: scalar,
    y: scalar,
) {
    _ptr.with_not_null_value(|layer| layer.borrow_mut().offset = Point::new(x, y));
}
