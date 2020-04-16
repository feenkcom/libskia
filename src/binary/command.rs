use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{BigEndian, WriteBytesExt};
use std::fmt::Debug;
use std::io::Cursor;

pub type CommandEndian = BigEndian;

pub trait Command: Debug + PartialEq {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self;
    fn write_to(&self, buffer: &mut Vec<u8>);
    fn execute(&self, context: &mut Context);

    fn as_vec(&self) -> Vec<u8> {
        let mut write_buffer = Vec::new();
        self.store_on(&mut write_buffer);
        write_buffer
    }

    fn command_type() -> CommandType;

    fn type_from(cursor: &mut Cursor<&[u8]>) -> CommandType {
        CommandType::type_from(cursor)
    }

    // A given slice must include the command type in the header
    fn from_slice<T>(slice: &[u8]) -> T
    where
        T: Command,
    {
        let mut read_cursor = Cursor::new(slice);
        let read_command_type = Self::type_from(&mut read_cursor);
        assert_eq!(read_command_type, Self::command_type());

        T::read_from(&mut read_cursor)
    }

    fn store_on(&self, buffer: &mut Vec<u8>) {
        buffer.write_u8(Self::command_type().into()).unwrap();
        self.write_to(buffer);
    }
}
