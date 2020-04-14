use binary::command::Command;
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use skia_safe::{scalar, Point, Rect};
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct FillRectangle {
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
}

impl FillRectangle {
    pub fn new(left: scalar, top: scalar, right: scalar, bottom: scalar) -> Self {
        FillRectangle {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl Command for FillRectangle {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        Self::new(
            cursor.read_f32::<NativeEndian>().unwrap(),
            cursor.read_f32::<NativeEndian>().unwrap(),
            cursor.read_f32::<NativeEndian>().unwrap(),
            cursor.read_f32::<NativeEndian>().unwrap(),
        )
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.write_f32::<NativeEndian>(self.left).unwrap();
        buffer.write_f32::<NativeEndian>(self.top).unwrap();
        buffer.write_f32::<NativeEndian>(self.right).unwrap();
        buffer.write_f32::<NativeEndian>(self.bottom).unwrap();
    }

    fn execute(&self, context: &mut Context) {
        let (canvas, paint) = context.peek_canvas_and_paint();
        let rect = Rect::new(self.left, self.top, self.right, self.bottom);
        canvas.draw_rect(rect, paint);
    }

    fn command_type(&self) -> CommandType {
        CommandType::FillRectangle
    }
}

#[test]
pub fn test_fill_rectangle() {
    let command = FillRectangle::new(10.0, 20.0, 30.0, 40.0);
    let mut buffer = command.as_vec();
    let read_command: FillRectangle = FillRectangle::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
