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
use binary::text::draw_glyphs::DrawGlyphs;
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

    DrawGlyphs,

    Undefined,
}

pub struct UndefinedCommand {

}

impl Command for UndefinedCommand {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self where Self: Sized {
        Self {

        }
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {

    }

    fn execute(&self, context: &mut Context) {

    }

    fn command_type(&self) -> CommandType where CommandType: Sized {
        CommandType::Undefined
    }
}

impl CommandType {
    pub fn type_from(cursor: &mut Cursor<&[u8]>) -> CommandType {
        CommandType::try_from(cursor.read_u8().unwrap()).unwrap()
    }

    pub fn command_from(&self, cursor: &mut Cursor<&[u8]>) -> Box<dyn Command> {
        match self {
            CommandType::NewPath => Box::new(NewPath::read_from(cursor)),
            CommandType::GetPath => Box::new(GetPath::read_from(cursor)),
            CommandType::MoveTo => Box::new(MoveTo::read_from(cursor)),
            CommandType::LineTo => Box::new(LineTo::read_from(cursor)),
            CommandType::Close => Box::new(Close::read_from(cursor)),
            CommandType::NewPaint => Box::new(NewPaint::read_from(cursor)),
            CommandType::PopPaint => Box::new(PopPaint::read_from(cursor)),
            CommandType::PaintSetRgba => Box::new(PaintSetRgba::read_from(cursor)),
            CommandType::FillRectangle => Box::new(FillRectangle::read_from(cursor)),
            CommandType::DrawGlyphs => Box::new(DrawGlyphs::read_from(cursor)),
            CommandType::Undefined => Box::new(UndefinedCommand::read_from(cursor)) }
    }

    pub fn execute(&self, context: &mut Context, cursor: &mut Cursor<&[u8]>) {
        self.command_from(cursor).execute(context);
    }
}
