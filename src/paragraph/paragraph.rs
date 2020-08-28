use boxer::array::BoxerArrayPointF32;
use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use boxer::point::BoxerPointF32;
use boxer::string::BoxerString;
use skia_safe::textlayout::{Paragraph, PlaceholderStyle, RectHeightStyle, RectWidthStyle, TextBoxes, PositionWithAffinity, LineMetricsVector, TextBox};
use skia_safe::{scalar, Canvas, Point};
use std::ops::{Range, Index};

#[derive(Debug)]
pub enum ParagraphPiece {
    Text(BoxerString),
    Placeholder(PlaceholderStyle),
}

pub struct ParagraphText {
    pieces: Vec<ParagraphPiece>,
    char_count: usize,
}

impl ParagraphText {
    pub fn new() -> Self {
        Self { pieces: vec![], char_count: 0 }
    }

    pub fn char_count(&self) -> usize {
        self.char_count
    }

    pub fn add_text(&mut self, text: BoxerString) {
        let char_count = text.char_count();
        self.pieces.push(ParagraphPiece::Text(text));
        self.char_count = self.char_count + char_count;
    }

    pub fn add_placeholder(&mut self, placeholder: PlaceholderStyle) {
        self.pieces.push(ParagraphPiece::Placeholder(placeholder));
    }

    pub fn get_glyph_range_for_char_range(&self, range: Range<usize>) -> Range<usize> {
        let first_range = self.get_glyph_range_for_char_index(range.start);
        let last_range = self.get_glyph_range_for_char_index(range.end - 1);

        first_range.start .. last_range.end
    }

    pub fn get_glyph_range_for_char_index(&self, index: usize) -> Range<usize> {
        let mut current_char_index: usize = 0;
        let mut glyph_range_start: usize = 0;
        let mut glyph_range_end: usize = 0;

        let mut buf = [0; 2];

        for piece in &self.pieces {
            match piece {
                ParagraphPiece::Text(string) => {
                    for ch in string.as_str().chars() {
                        let n = ch.encode_utf16(&mut buf).len();

                        glyph_range_start = glyph_range_end;
                        glyph_range_end = glyph_range_start + n;

                        if current_char_index == index {
                            return glyph_range_start .. glyph_range_end;
                        }

                        current_char_index = current_char_index + 1;
                    }
                },
                ParagraphPiece::Placeholder(_) => {
                    glyph_range_start = glyph_range_start + 1;
                    glyph_range_end = glyph_range_end + 1;
                },
            }
        }
        glyph_range_start .. glyph_range_end
    }

    pub fn get_char_offset_for_glyph_offset(&self, index: usize) -> usize {
        let mut current_char_index: usize = 0;
        let mut current_glyph_index: usize = 0;

        let mut buf = [0; 2];

        for piece in &self.pieces {
            if current_glyph_index == index {
                return current_char_index;
            }
            match piece {
                ParagraphPiece::Text(string) => {
                    for ch in string.as_str().chars() {
                        if current_glyph_index == index {
                            return current_char_index;
                        }

                        let n = ch.encode_utf16(&mut buf).len();

                        // left part of the char
                        if n == 2 && (current_glyph_index + 1) == index {
                            return current_char_index;
                        }

                        current_glyph_index = current_glyph_index + n;
                        current_char_index = current_char_index + 1;
                    }
                },
                ParagraphPiece::Placeholder(_) => {
                    current_glyph_index = current_glyph_index + 1;
                },
            }
        }
        current_char_index
    }


}

pub struct ParagraphWithText {
    paragraph: Paragraph,
    text: ParagraphText,
}

impl ParagraphWithText {
    pub fn new(paragraph: Paragraph, text: ParagraphText) -> Self {
        Self { paragraph, text }
    }

    pub fn layout(&mut self, width: scalar) {
        self.paragraph.layout(width);
    }

    pub fn paint(&self, canvas: &mut Canvas, p: impl Into<Point>) {
        self.paragraph.paint(canvas, p);
    }

    pub fn get_rects_for_range(
        &self,
        range: Range<usize>,
        rect_height_style: RectHeightStyle,
        rect_width_style: RectWidthStyle,
    ) -> TextBoxes {
        self.paragraph
            .get_rects_for_range(range, rect_height_style, rect_width_style)
    }

    pub fn get_rects_for_placeholders(&self) -> TextBoxes {
        self.paragraph.get_rects_for_placeholders()
    }

    pub fn get_glyph_position_at_coordinate(&self, p: impl Into<Point>) -> PositionWithAffinity {
        self.paragraph.get_glyph_position_at_coordinate(p)
    }

    pub fn get_line_metrics(&self) -> LineMetricsVector {
        self.paragraph.get_line_metrics()
    }

    pub fn line_number(&self) -> usize {
        self.paragraph.line_number()
    }

    pub fn height(&self) -> scalar {
        self.paragraph.height()
    }

    pub fn longest_line(&self) -> scalar {
        self.paragraph.longest_line()
    }

    pub fn char_count(&self) -> usize {
        self.text.char_count()
    }

    pub fn get_rects_for_char_range(&self, range: Range<usize>, rect_height_style: RectHeightStyle, rect_width_style: RectWidthStyle) -> TextBoxes {
        let glyph_range = self.text.get_glyph_range_for_char_range(range);
        self.get_rects_for_range(glyph_range, rect_height_style, rect_width_style)
    }

    pub fn get_line_index_for_char(&self, index: usize) -> usize {
        let glyph_range = self.text.get_glyph_range_for_char_index(index);

        for (index, line) in self.get_line_metrics().iter().enumerate() {
            if glyph_range.start < line.end_index {
                return index;
            }
        }
        self.line_number()
    }

    pub fn get_line_height(&self, index: usize) -> scalar {
        if self.line_number() == 0 && index == 0 {
            return self.height();
        }

        self.get_line_metrics().as_slice()[index].height as scalar
    }
}

#[no_mangle]
pub fn skia_paragraph_layout(paragraph_ptr: *mut ValueBox<ParagraphWithText>, width: scalar) {
    paragraph_ptr.with_not_null(|paragraph| {
        paragraph.layout(width);
    })
}

#[no_mangle]
pub fn skia_paragraph_paint(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    canvas_ptr: *mut ReferenceBox<Canvas>,
    x: scalar,
    y: scalar,
) {
    paragraph_ptr.with_not_null(|paragraph| {
        canvas_ptr.with_not_null(|canvas| {
            paragraph.paint(canvas, Point::new(x, y));
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_get_height(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |paragraph| paragraph.height())
}

#[no_mangle]
pub fn skia_paragraph_get_longest_line(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |paragraph| paragraph.longest_line())
}

#[no_mangle]
pub fn skia_paragraph_get_line_number(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> usize {
    paragraph_ptr.with_not_null_return(0, |paragraph| paragraph.line_number())
}

#[no_mangle]
pub fn skia_paragraph_get_char_count(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> usize {
    paragraph_ptr.with_not_null_return(0, |paragraph| paragraph.char_count())
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_placeholders(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
) -> *mut ValueBox<BoxerArrayPointF32> {
    paragraph_ptr.with_not_null_return(std::ptr::null_mut(), |paragraph| {
        let mut points: Vec<BoxerPointF32> = vec![];
        for text_box in paragraph.get_rects_for_placeholders().iter() {
            points.push(BoxerPointF32::new(text_box.rect.x(), text_box.rect.y()));
            points.push(BoxerPointF32::new(
                text_box.rect.right(),
                text_box.rect.bottom(),
            ));
        }
        let mut array = BoxerArrayPointF32::new();
        array.set_vector(points);
        ValueBox::new(array).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_glyph_range(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    start: usize,
    end: usize,
    rect_height_style: RectHeightStyle,
    rect_width_style: RectWidthStyle,
) -> *mut ValueBox<BoxerArrayPointF32> {
    paragraph_ptr.with_not_null_return(std::ptr::null_mut(), |paragraph| {
        let mut points: Vec<BoxerPointF32> = vec![];
        for text_box in paragraph
            .get_rects_for_range(start..end, rect_height_style, rect_width_style)
            .iter()
        {
            points.push(BoxerPointF32::new(text_box.rect.x(), text_box.rect.y()));
            points.push(BoxerPointF32::new(
                text_box.rect.right(),
                text_box.rect.bottom(),
            ));
        }
        let mut array = BoxerArrayPointF32::new();
        array.set_vector(points);
        ValueBox::new(array).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_char_range(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    start: usize,
    end: usize,
    rect_height_style: RectHeightStyle,
    rect_width_style: RectWidthStyle,
) -> *mut ValueBox<BoxerArrayPointF32> {
    paragraph_ptr.with_not_null_return(std::ptr::null_mut(), |paragraph| {
        let mut points: Vec<BoxerPointF32> = vec![];
        for text_box in paragraph
            .get_rects_for_char_range(start..end, rect_height_style, rect_width_style)
            .iter()
        {
            points.push(BoxerPointF32::new(text_box.rect.x(), text_box.rect.y()));
            points.push(BoxerPointF32::new(
                text_box.rect.right(),
                text_box.rect.bottom(),
            ));
        }
        let mut array = BoxerArrayPointF32::new();
        array.set_vector(points);
        ValueBox::new(array).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_print(paragraph_ptr: *mut ValueBox<ParagraphWithText>) {
    paragraph_ptr.with_not_null(|paragraph| {
        let line_metrics = paragraph.get_line_metrics();
        for (line, lm) in line_metrics.iter().enumerate() {
            println!(
                "line: {} width: {} end: {}",
                line + 1,
                lm.width,
                lm.end_index
            )
        }
    })
}

#[no_mangle]
pub fn skia_paragraph_get_glyph_position_at_coordinate(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    x: scalar,
    y: scalar,
) -> i32 {
    paragraph_ptr.with_not_null_return(0, |paragraph| {
        paragraph
            .get_glyph_position_at_coordinate(Point::new(x, y))
            .position
    })
}

#[no_mangle]
pub fn skia_paragraph_get_char_position_at_coordinate(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    x: scalar,
    y: scalar,
) -> usize {
    paragraph_ptr.with_not_null_return(0, |paragraph| {
        let position = paragraph.get_glyph_position_at_coordinate(Point::new(x, y));
        if position.position < 0 {
            return 0;
        }
        paragraph.text.get_char_offset_for_glyph_offset(position.position as usize)
    })
}

#[no_mangle]
pub fn skia_paragraph_get_line_index_for_char(paragraph_ptr: *mut ValueBox<ParagraphWithText>, index: usize) -> usize {
    paragraph_ptr.with_not_null_return(0, |paragraph| {
        paragraph.get_line_index_for_char(index)
    })
}

#[no_mangle]
pub fn skia_paragraph_get_line_height(paragraph_ptr: *mut ValueBox<ParagraphWithText>, index: usize) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |paragraph| {
        paragraph.get_line_height(index)
    })
}

#[no_mangle]
pub fn skia_paragraph_drop(ptr: *mut ValueBox<ParagraphWithText>) {
    ptr.drop();
}
