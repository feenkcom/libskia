use binary::command::{Command, CommandEndian};
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use skia_safe::{Path, Point};
use std::io::Cursor;

#[derive(PartialEq, Default, Debug)]
#[repr(C)]
pub(crate) struct GetPath {
    id: u64,
}

impl Command for GetPath {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        let id = cursor.read_u64::<CommandEndian>().unwrap();
        Self { id }
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.write_u64::<CommandEndian>(self.id).unwrap();
    }

    fn execute(&self, context: &mut Context) {
        context.push_path(Path::new());
    }

    fn command_type(&self) -> CommandType {
        CommandType::GetPath
    }
}

#[test]
pub fn test_get_path() {
    let context = Context::new();

    let command = GetPath { id: 1 };
    let mut buffer = command.as_vec();
    let read_command: GetPath = GetPath::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
