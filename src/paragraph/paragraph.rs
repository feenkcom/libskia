use boxer::array::BoxerArrayPointF32;
use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use boxer::point::BoxerPointF32;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::textlayout::{
    Affinity, LineMetricsVector, Paragraph, PlaceholderStyle, PositionWithAffinity,
    RectHeightStyle, RectWidthStyle, TextBoxes,
};
use skia_safe::{scalar, Canvas, Point, Rect};
use std::ops::Range;

pub type TabSize = usize;
pub type CharLength = usize;

#[derive(Debug)]
pub enum ParagraphPiece {
    Text(BoxerString),
    Placeholder(PlaceholderStyle, CharLength),
}

pub struct ParagraphText {
    pieces: Vec<ParagraphPiece>,
    char_count: usize,
    tab_size: TabSize,
}

impl ParagraphText {
    pub fn new(tab_size: TabSize) -> Self {
        Self {
            pieces: vec![],
            char_count: 0,
            tab_size,
        }
    }

    pub fn char_count(&self) -> usize {
        self.char_count
    }

    pub fn add_text(&mut self, text: BoxerString) {
        let char_count = text.char_count();
        self.pieces.push(ParagraphPiece::Text(text));
        self.char_count = self.char_count + char_count;
    }

    pub fn add_placeholder(&mut self, placeholder: PlaceholderStyle, char_length: CharLength) {
        self.pieces
            .push(ParagraphPiece::Placeholder(placeholder, char_length));
        self.char_count = self.char_count + char_length;
    }

    pub fn get_glyph_range_for_char_range(&self, range: Range<usize>) -> Range<usize> {
        let first_range = self.get_glyph_offset_for_char_offset(range.start);
        let last_range = self.get_glyph_offset_for_char_offset(range.end);

        first_range..last_range
    }

    fn get_char_len(&self, ch: char, buf: &mut [u16]) -> usize {
        match ch {
            '\t' => self.tab_size * ch.encode_utf16(buf).len(),
            _ => ch.encode_utf16(buf).len(),
        }
    }

    pub fn get_glyph_offset_for_char_offset(&self, index: usize) -> usize {
        let mut current_char_index: usize = 0;
        let mut current_glyph_index: usize = 0;

        let mut buf = [0; 2];

        for piece in &self.pieces {
            if current_char_index >= index {
                return current_glyph_index;
            }
            match piece {
                ParagraphPiece::Text(string) => {
                    for ch in string.as_str().chars() {
                        if current_char_index >= index {
                            return current_glyph_index;
                        }
                        let n = self.get_char_len(ch, &mut buf);

                        current_glyph_index = current_glyph_index + n;
                        current_char_index = current_char_index + 1;
                    }
                }
                ParagraphPiece::Placeholder(_, char_length) => {
                    current_glyph_index = current_glyph_index + 1;
                    current_char_index = current_char_index + *char_length;
                }
            }
        }
        current_glyph_index
    }

    pub fn get_char_offset_for_glyph_offset(&self, index: usize) -> usize {
        let mut current_char_index: usize = 0;
        let mut current_glyph_index: usize = 0;

        let mut buf = [0; 2];

        for piece in &self.pieces {
            if current_glyph_index >= index {
                return current_char_index;
            }
            match piece {
                ParagraphPiece::Text(string) => {
                    for ch in string.as_str().chars() {
                        if current_glyph_index >= index {
                            return current_char_index;
                        }

                        let n = self.get_char_len(ch, &mut buf);
                        let next_glyph_index = current_glyph_index + n;

                        // looks like the target glyph index is within this char
                        if next_glyph_index >= index {
                            let target_glyph_offset = index - current_glyph_index;

                            // left part of the char
                            if n > 1 {
                                if target_glyph_offset <= (n / 2) {
                                    return current_char_index;
                                } else {
                                    return current_char_index + 1;
                                }
                            }
                        }

                        current_glyph_index = next_glyph_index;
                        current_char_index = current_char_index + 1;
                    }
                }
                ParagraphPiece::Placeholder(_, char_length) => {
                    current_glyph_index = current_glyph_index + 1;
                    current_char_index = current_char_index + *char_length;
                }
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

    pub fn get_coordinate_outside_placeholder(
        &self,
        p: impl Into<Point>,
        global_affinity: Option<Affinity>,
    ) -> Point {
        let coordinate: Point = p.into();

        match self.get_placeholder_at_coordinate(coordinate) {
            None => coordinate,
            Some((_placeholder, rect, affinity)) => {
                let local_affinity = match global_affinity {
                    None => affinity,
                    Some(affinity) => affinity,
                };

                let new_coordinate = match local_affinity {
                    Affinity::Upstream => Point::new(rect.right + 1.0, coordinate.y),
                    Affinity::Downstream => Point::new(rect.left - 1.0, coordinate.y),
                };
                self.get_coordinate_outside_placeholder(new_coordinate, Some(affinity))
            }
        }
    }

    pub fn get_glyph_position_at_coordinate(&self, p: impl Into<Point>) -> PositionWithAffinity {
        let point: Point = p.into();
        let coordinate = self.get_coordinate_outside_placeholder(point, None);
        self.paragraph.get_glyph_position_at_coordinate(coordinate)
    }

    pub fn get_placeholder_at_index(&self, index: usize) -> &PlaceholderStyle {
        let placeholders: Vec<&PlaceholderStyle> =
            (self.text.pieces.iter().map(|piece| match piece {
                ParagraphPiece::Text(_) => None,
                ParagraphPiece::Placeholder(placeholder, _) => Some(placeholder),
            }))
            .filter(|piece| piece.is_some())
            .map(|placeholder| placeholder.unwrap())
            .collect();

        placeholders[index]
    }

    fn rect_contains(rect: &Rect, point: &Point) -> bool {
        if point.x < rect.left || point.y < rect.top {
            return false;
        }
        if point.x > rect.right || point.y > rect.bottom {
            return false;
        }
        true
    }

    pub fn get_placeholder_at_coordinate(
        &self,
        p: impl Into<Point>,
    ) -> Option<(&PlaceholderStyle, Rect, Affinity)> {
        let point: Point = p.into();

        let mut placeholder_index: usize = 0;

        for rect in self.get_rects_for_placeholders().iter() {
            if Self::rect_contains(&rect.rect, &point) {
                let affinity = if point.x < rect.rect.center_x() {
                    Affinity::Downstream
                } else {
                    Affinity::Upstream
                };
                let result = Some((
                    self.get_placeholder_at_index(placeholder_index),
                    rect.rect,
                    affinity,
                ));
                return result;
            }
            placeholder_index = placeholder_index + 1;
        }

        None
    }

    pub fn get_char_position_at_coordinate(&self, p: impl Into<Point>) -> usize {
        let position_with_affinity = self.get_glyph_position_at_coordinate(p);
        if position_with_affinity.position < 0 {
            return 0;
        }
        self.text
            .get_char_offset_for_glyph_offset(position_with_affinity.position as usize)
    }

    pub fn get_line_metrics(&self) -> LineMetricsVector {
        self.paragraph.get_line_metrics()
    }

    pub fn line_number(&self) -> usize {
        self.paragraph.line_number()
    }

    pub fn max_width(&self) -> scalar {
        self.paragraph.max_width()
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

    pub fn get_rects_for_char_range(
        &self,
        char_range: Range<usize>,
        rect_height_style: RectHeightStyle,
        rect_width_style: RectWidthStyle,
    ) -> TextBoxes {
        let glyph_range = self.text.get_glyph_range_for_char_range(char_range.clone());
        self.get_rects_for_range(glyph_range, rect_height_style, rect_width_style)
    }

    pub fn get_line_index_for_char(&self, index: usize) -> usize {
        let glyph_offset = self.text.get_glyph_offset_for_char_offset(index);

        for (index, line) in self.get_line_metrics().iter().enumerate() {
            if glyph_offset <= line.end_index {
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
pub fn skia_paragraph_get_max_width(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |paragraph| paragraph.max_width())
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
        let position_with_affinity = paragraph.get_glyph_position_at_coordinate(Point::new(x, y));
        position_with_affinity.position
    })
}

#[no_mangle]
pub fn skia_paragraph_get_char_position_at_coordinate(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    x: scalar,
    y: scalar,
) -> usize {
    paragraph_ptr.with_not_null_return(0, |paragraph| {
        paragraph.get_char_position_at_coordinate(Point::new(x, y))
    })
}

#[no_mangle]
pub fn skia_paragraph_get_glyph_range_for_char_range(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    start: usize,
    end: usize,
) -> *mut ValueBox<Range<usize>> {
    paragraph_ptr.with_not_null_return(std::ptr::null_mut(), |paragraph| {
        ValueBox::new(paragraph.text.get_glyph_range_for_char_range(start..end)).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_get_line_index_for_char(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    index: usize,
) -> usize {
    paragraph_ptr.with_not_null_return(0, |paragraph| paragraph.get_line_index_for_char(index))
}

#[no_mangle]
pub fn skia_paragraph_get_line_height(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    index: usize,
) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |paragraph| paragraph.get_line_height(index))
}

#[no_mangle]
pub fn skia_paragraph_drop(ptr: &mut *mut ValueBox<ParagraphWithText>) {
    drop!(ptr);
}
