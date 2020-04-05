use binary::command::Command;
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use skia_safe::{Path, Point};
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct Close {}

impl Close {
    pub fn new() -> Self {
        Close {}
    }
}

impl Command for Close {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        Self {}
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {}

    fn execute(&self, context: &mut Context) {
        let path = context.peek_path_mut();
        path.close();
    }

    fn command_type() -> CommandType {
        CommandType::Close
    }
}

#[test]
pub fn test_close() {
    let command = Close {};
    let mut buffer = command.as_vec();
    let read_command: Close = Close::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
