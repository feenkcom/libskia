use binary::command::Command;
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use skia_safe::Paint;
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct PaintSetRgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl PaintSetRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        PaintSetRgba { r, g, b, a }
    }
}

impl Command for PaintSetRgba {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        Self::new(
            cursor.read_u8().unwrap(),
            cursor.read_u8().unwrap(),
            cursor.read_u8().unwrap(),
            cursor.read_u8().unwrap(),
        )
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.write_u8(self.r).unwrap();
        buffer.write_u8(self.g).unwrap();
        buffer.write_u8(self.b).unwrap();
        buffer.write_u8(self.a).unwrap();
    }

    fn execute(&self, context: &mut Context) {
        let paint = context.peek_paint_mut();
        paint.set_argb(self.a, self.r, self.g, self.b);
    }

    fn command_type(&self) -> CommandType {
        CommandType::PaintSetRgba
    }
}

#[test]
pub fn test_paint_set_rgba() {
    let context = Context::new();

    let command = PaintSetRgba::new(50, 100, 200, 255);
    let mut buffer = command.as_vec();
    let read_command: PaintSetRgba = PaintSetRgba::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
