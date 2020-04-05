use binary::command::{Command, CommandEndian};
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use skia_safe::{Path, Point};
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct LineTo {
    absolute: bool,
    x: f32,
    y: f32,
}

impl LineTo {
    pub fn absolute(x: f32, y: f32) -> Self {
        LineTo {
            absolute: true,
            x,
            y,
        }
    }

    pub fn relative(x: f32, y: f32) -> Self {
        LineTo {
            absolute: false,
            x,
            y,
        }
    }
}

impl Command for LineTo {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        let absolute = cursor.read_uint::<CommandEndian>(1).unwrap() as u64 == true as u64;
        let x = cursor.read_f32::<CommandEndian>().unwrap();
        let y = cursor.read_f32::<CommandEndian>().unwrap();

        Self { absolute, x, y }
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer
            .write_uint::<CommandEndian>(self.absolute as u64, 1)
            .unwrap();
        buffer.write_f32::<CommandEndian>(self.x).unwrap();
        buffer.write_f32::<CommandEndian>(self.y).unwrap();
    }

    fn execute(&self, context: &mut Context) {
        let path = context.peek_path_mut();
        if self.absolute {
            path.line_to(Point::new(self.x, self.y));
        } else {
            path.r_line_to(Point::new(self.x, self.y));
        }
    }
    fn command_type() -> CommandType {
        CommandType::LineTo
    }
}

#[test]
pub fn test_line_to() {
    let command = LineTo {
        absolute: true,
        x: 10.0,
        y: 20.0,
    };
    let mut buffer = command.as_vec();
    let read_command: LineTo = LineTo::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
