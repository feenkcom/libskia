use binary::command::Command;
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use skia_safe::Paint;
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct PopPaint {}

impl PopPaint {
    pub fn new() -> Self {
        PopPaint {}
    }
}

impl Command for PopPaint {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        Self {}
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {}

    fn execute(&self, context: &mut Context) {
        context.pop_paint();
    }

    fn command_type() -> CommandType {
        CommandType::PopPaint
    }
}

#[test]
pub fn test_pop_paint() {
    let context = Context::new();

    let command = PopPaint::new();
    let mut buffer = command.as_vec();
    let read_command: PopPaint = PopPaint::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
