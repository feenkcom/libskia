use binary::command::{Command, CommandEndian};
use binary::context::Context;
use binary::fill::fill_rectangle::FillRectangle;
use binary::paint::new_paint::NewPaint;
use binary::paint::pop_paint::PopPaint;
use binary::paint::set_rgba::PaintSetRgba;
use binary::path::close::Close;
use binary::path::get_path::GetPath;
use binary::path::line_to::LineTo;
use binary::path::move_to::MoveTo;
use binary::path::new_path::NewPath;
use byteorder::{NativeEndian, ReadBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::io::Cursor;

pub type CommandTypeSize = u8;

#[derive(Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandType {
    NewPath,
    GetPath,
    MoveTo,
    LineTo,
    Close,

    NewPaint,
    PopPaint,
    PaintSetRgba,

    FillRectangle,
}

impl CommandType {
    pub fn type_from(cursor: &mut Cursor<&[u8]>) -> CommandType {
        CommandType::try_from(cursor.read_u8().unwrap()).unwrap()
    }

    pub fn execute(&self, context: &mut Context, cursor: &mut Cursor<&[u8]>) {
        match self {
            CommandType::NewPath => NewPath::read_from(cursor).execute(context),
            CommandType::GetPath => GetPath::read_from(cursor).execute(context),
            CommandType::MoveTo => MoveTo::read_from(cursor).execute(context),
            CommandType::LineTo => LineTo::read_from(cursor).execute(context),
            CommandType::Close => Close::read_from(cursor).execute(context),
            CommandType::NewPaint => NewPaint::read_from(cursor).execute(context),
            CommandType::PopPaint => PopPaint::read_from(cursor).execute(context),
            CommandType::PaintSetRgba => PaintSetRgba::read_from(cursor).execute(context),
            CommandType::FillRectangle => FillRectangle::read_from(cursor).execute(context),
        }
    }
}
