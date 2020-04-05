use binary::commands::CommandType;
use skia_safe::{Canvas, Paint, Path, Surface};
use std::io::Cursor;

pub struct Context {
    paths: Vec<Path>,
    paints: Vec<Paint>,

    current_path: Vec<Path>,
    current_paint: Vec<Paint>,

    surface: Surface,
}

impl Context {
    pub fn new() -> Self {
        Self::new_surface(Surface::new_raster_n32_premul((1, 1)).unwrap())
    }

    pub fn new_surface(surface: Surface) -> Self {
        Context {
            paths: vec![],
            paints: vec![],

            current_paint: vec![],
            current_path: vec![],

            surface,
        }
    }

    pub fn execute(&mut self, buffer: &[u8]) {
        let len = buffer.len();
        let mut cursor = Cursor::new(buffer);

        while (cursor.position() as usize) < len - 1 {
            let command_type = CommandType::type_from(&mut cursor);
            command_type.execute(self, &mut cursor);
        }
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        self.surface.canvas()
    }

    pub fn surface(self) -> Surface {
        self.surface
    }

    pub fn push_path(&mut self, path: Path) {
        self.current_path.push(path);
    }

    pub fn with_path<Block>(&mut self, block: Block)
    where
        Block: FnOnce(&mut Path),
    {
        let path = self.current_path.last_mut().unwrap();
        block(path);
    }

    pub fn peek_path(&self) -> &Path {
        self.current_path.last().unwrap()
    }

    pub fn peek_path_mut(&mut self) -> &mut Path {
        self.current_path.last_mut().unwrap()
    }

    pub fn push_paint(&mut self, paint: Paint) {
        self.current_paint.push(paint);
    }

    pub fn pop_paint(&mut self) {
        self.current_paint.pop();
    }

    pub fn peek_paint(&self) -> &Paint {
        self.current_paint.last().unwrap()
    }

    pub fn peek_paint_mut(&mut self) -> &mut Paint {
        self.current_paint.last_mut().unwrap()
    }

    pub fn peek_canvas_and_paint(&mut self) -> (&mut Canvas, &Paint) {
        (self.surface.canvas(), self.current_paint.last().unwrap())
    }
}
