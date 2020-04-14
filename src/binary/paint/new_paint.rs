use binary::command::Command;
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use skia_safe::Paint;
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct NewPaint {}

impl NewPaint {
    pub fn new() -> Self {
        NewPaint {}
    }
}

impl Command for NewPaint {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        Self {}
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {}

    fn execute(&self, context: &mut Context) {
        context.push_paint(Paint::default());
    }

    fn command_type(&self) -> CommandType {
        CommandType::NewPaint
    }
}

#[test]
pub fn test_new_paint() {
    let context = Context::new();

    let command = NewPaint::new();
    let mut buffer = command.as_vec();
    let read_command: NewPaint = NewPaint::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
