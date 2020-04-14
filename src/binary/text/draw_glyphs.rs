use binary::command::{Command, CommandEndian};
use binary::commands::CommandType;
use binary::context::Context;
use binary::font::font_face::BinaryFontFace;
use byteorder::{ReadBytesExt, WriteBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use skia_safe::{scalar, Path, Point, TextBlob, TextEncoding, ImageInfo, Surface};
use std::convert::TryFrom;
use std::io::{Cursor, Read, Write};

#[derive(Copy, Clone, PartialEq, Eq, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum BinaryTextEncoding {
    UTF8,
    UTF16,
    UTF32,
    GlyphId,
}

#[derive(PartialEq, Debug)]
#[repr(C)]
pub(crate) struct DrawGlyphs {
    encoding: BinaryTextEncoding,
    glyphs: Vec<u8>,
    font_face: BinaryFontFace,
    font_size: scalar,
    x: scalar,
    y: scalar,
    width: i32,
    height: i32,
    pixels: Vec<u8>,
    is_cached: bool
}

impl DrawGlyphs {
    pub fn new_utf8(glyphs: Vec<u8>) -> Self {
        Self {
            encoding: BinaryTextEncoding::UTF8,
            glyphs,
            font_face: BinaryFontFace::default(),
            font_size: 14.0,
            x: 0.0,
            y: 0.0
        }
    }
}

impl Command for DrawGlyphs {
    fn read_from(cursor: &mut Cursor<&[u8]>) -> Self {
        let encoding = BinaryTextEncoding::try_from(cursor.read_u8().unwrap()).unwrap();
        let length = cursor.read_u64::<CommandEndian>().unwrap();

        let mut glyphs = vec![0u8; length as usize];
        cursor.read_exact(glyphs.as_mut_slice());

        let font_face = BinaryFontFace::read_from(cursor);
        let font_size = cursor.read_f32::<CommandEndian>().unwrap();

        let x = cursor.read_f32::<CommandEndian>().unwrap();
        let y = cursor.read_f32::<CommandEndian>().unwrap();

        Self {
            encoding,
            glyphs,
            font_face,
            font_size,
            x,
            y
        }
    }

    fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.write_u8(self.encoding.into()).unwrap();
        buffer
            .write_u64::<CommandEndian>(self.glyphs.len() as u64)
            .unwrap();
        buffer.write(self.glyphs.as_slice()).unwrap();

        self.font_face.write_to(buffer);
        buffer.write_f32::<CommandEndian>(self.font_size).unwrap();
        buffer.write_f32::<CommandEndian>(self.x).unwrap();
        buffer.write_f32::<CommandEndian>(self.y).unwrap();
    }

    fn execute(&mut self, context: &mut Context) {
        let image_info = ImageInfo::new(
            (self.width, self.height),
            crate::ColorType::RGBA8888,
            crate::AlphaType::Unpremul,
            None,
        );
        let min_row_bytes = image_info.min_row_bytes();

        let mut surface = Surface::new_raster_direct(
            &image_info,
            self.pixels.as_mut_slice(),
            Some(min_row_bytes),
            None,
        )
        .unwrap();

        let (canvas, paint, font) = context.peek_canvas_and_paint_and_find_font(self.font_face.clone(), self.font_size);

        match TextBlob::from_text(self.glyphs.as_slice(), TextEncoding::UTF8, font) {
            None => {},
            Some(text_blob) => {
                surface.canvas().draw_text_blob(text_blob, Point::new(self.x, self.y), paint);
            }
        }

        canvas.draw_image(surface.image_snapshot(),)
    }

    fn command_type(&self) -> CommandType {
        CommandType::DrawGlyphs
    }
}

#[test]
pub fn test_draw_glyphs() {
    let command = DrawGlyphs::new_utf8(vec![1, 2, 3, 4, 5]);
    let mut buffer = command.as_vec();
    let read_command: DrawGlyphs = DrawGlyphs::from_slice(buffer.as_slice());
    assert_eq!(read_command, command);

    dbg!(buffer);
}
