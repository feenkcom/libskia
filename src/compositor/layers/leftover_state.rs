use compositor::compositor::RasterizerContext;
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use skia_safe::{Canvas, ClipOp, Matrix, Path, Picture, Point, RRect, Rect, Vector};
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

    pub fn update_rasterizer_context(&self, context: RasterizerContext) -> RasterizerContext {
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

    fn draw_on(&mut self, context: RasterizerContext, canvas: &mut Canvas) {
        let count = canvas.save();
        self.apply_on_canvas(canvas);
        for layer in &self.layers {
            layer
                .borrow_mut()
                .draw_on(self.update_rasterizer_context(context), canvas);
        }
        canvas.restore_to_count(count);
    }

    fn take_picture_to_rasterize(
        &mut self,
        context: RasterizerContext,
        mut pictures: &mut Vec<PictureToRasterize>,
    ) {
        for mut layer in &self.layers {
            layer
                .borrow_mut()
                .take_picture_to_rasterize(self.update_rasterizer_context(context), pictures);
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
