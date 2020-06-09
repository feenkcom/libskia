use boxer::boxes::{ValueBox, ValueBoxPointer};
use compositor::compositor::{CompositorContext, MatrixContext};
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use skia_safe::{scalar, Canvas, ClipOp, Matrix, Path, Picture, Point, RRect, Rect, Vector};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub struct StateCommand {
    command_type: StateCommandType,
    offset: Vector,
}

pub enum StateCommandType {
    ClipRect(Rect),
    ClipPath(Path),
    ClipRRect(RRect),
    Transform(Matrix),
}

impl Debug for StateCommandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut formatter = f.debug_struct("StateCommandType");

        let formatted = match self {
            StateCommandType::ClipRect(rect) => {
                formatter.field("type", &String::from("ClipRect"));
                formatter.field("rect", rect)
            }
            StateCommandType::ClipPath(path) => formatter.field("type", &String::from("ClipPath")),
            StateCommandType::ClipRRect(rrect) => {
                formatter.field("type", &String::from("ClipRRect"));
                formatter.field("rrect", rrect.rect())
            }
            StateCommandType::Transform(matrix) => {
                formatter.field("type", &String::from("Transform"));
                formatter.field("rect", matrix)
            }
        };
        formatted.finish()
    }
}

impl StateCommand {
    pub fn clip_rect(rect: Rect, offset: Vector) -> Self {
        StateCommand {
            command_type: StateCommandType::ClipRect(rect),
            offset,
        }
    }

    pub fn clip_rrect(rrect: RRect, offset: Vector) -> Self {
        StateCommand {
            command_type: StateCommandType::ClipRRect(rrect),
            offset,
        }
    }

    pub fn clip_path(path: Path, offset: Vector) -> Self {
        StateCommand {
            command_type: StateCommandType::ClipPath(path),
            offset,
        }
    }

    pub fn transform(matrix: Matrix, offset: Vector) -> Self {
        StateCommand {
            command_type: StateCommandType::Transform(matrix),
            offset,
        }
    }
}

#[derive(Debug)]
pub struct LeftoverStateLayer {
    pub layers: Vec<Rc<RefCell<dyn Layer>>>,
    pub commands: Vec<StateCommand>,
}

impl LeftoverStateLayer {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            commands: vec![],
        }
    }

    pub fn clip_rect(&mut self, rect: Rect, offset: Vector) {
        self.commands.push(StateCommand::clip_rect(rect, offset));
    }

    pub fn clip_rrect(&mut self, rrect: RRect, offset: Vector) {
        self.commands.push(StateCommand::clip_rrect(rrect, offset));
    }

    pub fn clip_path(&mut self, path: Path, offset: Vector) {
        self.commands.push(StateCommand::clip_path(path, offset));
    }

    pub fn transform(&mut self, matrix: Matrix, offset: Vector) {
        self.commands.push(StateCommand::transform(matrix, offset));
    }

    pub fn update_rasterizer_context(&self, context: MatrixContext) -> MatrixContext {
        let mut current_context = context;
        for command in &self.commands {
            match &command.command_type {
                StateCommandType::Transform(matrix) => {
                    current_context = current_context.translate(command.offset).concat(matrix)
                }
                _ => {}
            }
        }
        current_context
    }

    pub fn apply_on_canvas(&self, canvas: &mut Canvas) {
        for command in &self.commands {
            match &command.command_type {
                StateCommandType::ClipRect(rect) => {
                    canvas.clip_rect(rect.with_offset(command.offset), ClipOp::Intersect, true);
                }
                StateCommandType::ClipPath(path) => {
                    canvas.clip_path(&path.with_offset(command.offset), ClipOp::Intersect, true);
                }
                StateCommandType::ClipRRect(rrect) => {
                    canvas.clip_rrect(rrect.with_offset(command.offset), ClipOp::Intersect, true);
                }
                StateCommandType::Transform(matrix) => {
                    canvas.translate(command.offset);
                    canvas.concat(matrix);
                }
            }
        }
    }
}

impl Layer for LeftoverStateLayer {
    fn add_layer(&mut self, layer: Rc<RefCell<dyn Layer>>) {
        self.layers.push(layer);
    }

    fn count_layers(&self) -> usize {
        self.layers.len()
    }

    fn prepare(&mut self, context: &mut CompositorContext) {
        let matrix_context = self
            .update_rasterizer_context(MatrixContext::new_matrix(context.current_matrix().clone()));
        context.push_matrix(matrix_context.matrix);

        for layer in &self.layers {
            layer.borrow_mut().prepare(context);
        }
        context.pop_matrix();
    }

    fn draw(&mut self, context: &mut CompositorContext) {
        let matrix_context = self
            .update_rasterizer_context(MatrixContext::new_matrix(context.current_matrix().clone()));
        context.push_matrix(matrix_context.matrix);

        let count = context.canvas().save();
        self.apply_on_canvas(context.canvas());

        for layer in &self.layers {
            layer.borrow_mut().draw(context);
        }
        context.canvas().restore_to_count(count);

        context.pop_matrix();
    }
}

#[no_mangle]
pub fn skia_leftover_state_layer_new() -> *mut ValueBox<Rc<RefCell<dyn Layer>>> {
    let layer: Rc<RefCell<dyn Layer>> = Rc::new(RefCell::new(LeftoverStateLayer::new()));
    ValueBox::new(layer).into_raw()
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_rect(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().clip_rect(
            Rect::new(left, top, right, bottom),
            Vector::new(offset_x, offset_y),
        );
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_rrect(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
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
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().clip_rrect(
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
        );
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_clip_path(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    _ptr_path: *mut ValueBox<Path>,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        _ptr_path.with_not_null_value(|path| {
            layer
                .borrow_mut()
                .clip_path(path, Vector::new(offset_x, offset_y));
        })
    })
}

#[no_mangle]
pub fn skia_leftover_state_layer_transform(
    _ptr: *mut ValueBox<Rc<RefCell<LeftoverStateLayer>>>,
    scale_x: scalar,
    skew_x: scalar,
    trans_x: scalar,
    skew_y: scalar,
    scale_y: scalar,
    trans_y: scalar,
    persp_0: scalar,
    persp_1: scalar,
    persp_2: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    _ptr.with_not_null_value(|layer| {
        layer.borrow_mut().transform(
            Matrix::new_all(
                scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, persp_0, persp_1, persp_2,
            ),
            Vector::new(offset_x, offset_y),
        );
    })
}
