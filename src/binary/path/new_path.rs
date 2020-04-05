use binary::command::Command;
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use skia_safe::{Path, Point};
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct NewPath {}

impl NewPath {
    pub fn new() -> Self {
        NewPath {}
    }
}

impl Command for NewPath {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        Self {}
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {}

    fn execute(&self, context: &mut Context) {
        context.push_path(Path::new());
    }

    fn command_type() -> CommandType {
        CommandType::NewPath
    }
}

#[test]
pub fn test_new_path() {
    let context = Context::new();

    let command = NewPath::new();
    let mut buffer = command.as_vec();
    let read_command: NewPath = NewPath::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}