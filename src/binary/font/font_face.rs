use binary::command::{Command, CommandEndian};
use binary::commands::CommandType;
use binary::context::Context;
use byteorder::{ReadBytesExt, WriteBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::io::{Cursor, Read, Write};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BinaryFontFaceSlant {
    Upright = 0,
    Italic = 1,
    Oblique = 2,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BinaryFontFaceWidth {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct BinaryFontFace {
    family_name: String,
    weight: i32,
    slant: BinaryFontFaceSlant,
    width: BinaryFontFaceWidth,
}

impl Command for BinaryFontFace {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        let mut buffer = Vec::<u8>::with_capacity(20);
        while {
            let byte = cursor.read_u8().unwrap();
            if byte != 0 {
                buffer.push(byte);
                true
            } else {
                false
            }
        } {}
        let mut family_name = unsafe { String::from_utf8_unchecked(buffer) };

        let weight = cursor.read_i32::<CommandEndian>().unwrap();
        let slant = BinaryFontFaceSlant::try_from(cursor.read_u8().unwrap()).unwrap();
        let width = BinaryFontFaceWidth::try_from(cursor.read_u8().unwrap()).unwrap();

        Self {
            family_name,
            weight,
            slant,
            width,
        }
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.write_all(self.family_name.as_bytes()).unwrap();
        buffer.write_u8(0u8);
        buffer.write_i32::<CommandEndian>(self.weight).unwrap();
        buffer.write_u8(self.slant.into()).unwrap();
        buffer.write_u8(self.width.into()).unwrap();
    }

    fn execute(&self, context: &mut Context) {}

    fn command_type(&self) -> CommandType {
        CommandType::Undefined
    }
}

impl Default for BinaryFontFace {
    fn default() -> Self {
        Self {
            family_name: "Arial".to_string(),
            weight: 500,
            slant: BinaryFontFaceSlant::Upright,
            width: BinaryFontFaceWidth::Normal,
        }
    }
}

#[test]
pub fn test_font_face() {
    let command = BinaryFontFace::default();
    let mut buffer = command.as_vec();
    let read_command: BinaryFontFace = BinaryFontFace::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
