use std::any::type_name;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use array_box::ArrayBox;
use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::textlayout::{
    Affinity, LineMetrics, Paragraph, PlaceholderStyle, PositionWithAffinity, RectHeightStyle,
    RectWidthStyle, TextBox,
};
use skia_safe::{scalar, Canvas, Point, Rect};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

pub type TabSize = usize;
pub type CharLength = usize;

#[derive(Debug, Clone)]
pub enum ParagraphPiece {
    Text(StringBox),
    Placeholder(PlaceholderStyle, CharLength),
}

#[derive(Debug, Clone)]
pub struct ParagraphText {
    pub(crate) pieces: Vec<ParagraphPiece>,
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

    pub fn tab_size(&self) -> TabSize {
        self.tab_size
    }

    pub fn add_text(&mut self, text: StringBox) {
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

    pub fn get_glyph_offset_for_char_offset(&self, offset: usize) -> usize {
        let mut current_char_index: usize = 0;
        let mut current_glyph_index: usize = 0;

        let mut buf = [0; 2];

        for piece in &self.pieces {
            if current_char_index >= offset {
                return current_glyph_index;
            }
            match piece {
                ParagraphPiece::Text(string) => {
                    for ch in string.as_str().chars() {
                        if current_char_index >= offset {
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
                                return if target_glyph_offset <= (n / 2) {
                                    current_char_index
                                } else {
                                    current_char_index + 1
                                };
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

#[derive(Debug, Clone)]
pub struct ParagraphWithText {
    pub(crate) paragraph: Rc<RefCell<Paragraph>>,
    pub(crate) text: ParagraphText,
}

impl ParagraphWithText {
    pub fn new(paragraph: Paragraph, text: ParagraphText) -> Self {
        Self {
            paragraph: Rc::new(RefCell::new(paragraph)),
            text,
        }
    }

    pub fn layout(&mut self, width: scalar) {
        self.paragraph.borrow_mut().layout(width);
    }

    pub fn paint(&self, canvas: &Canvas, p: impl Into<Point>) {
        self.paragraph.borrow().paint(canvas, p);
    }

    pub fn get_rects_for_range(
        &self,
        range: Range<usize>,
        rect_height_style: RectHeightStyle,
        rect_width_style: RectWidthStyle,
    ) -> Vec<TextBox> {
        self.paragraph
            .borrow()
            .get_rects_for_range(range, rect_height_style, rect_width_style)
    }

    pub fn get_rects_for_placeholders(&self) -> Vec<TextBox> {
        self.paragraph.borrow().get_rects_for_placeholders()
    }

    pub fn get_coordinate_outside_placeholder(
        &self,
        p: impl Into<Point>,
        global_affinity: Option<Affinity>,
    ) -> Point {
        let coordinate: Point = p.into();

        trace!(
            "[get_coordinate_outside_placeholder] at point {:?} with global affinity {:?}",
            coordinate,
            global_affinity
        );

        match self.get_placeholder_at_coordinate(coordinate) {
            None => coordinate,
            Some((_placeholder, rect, placeholder_affinity)) => {
                let local_affinity = match global_affinity {
                    None => placeholder_affinity,
                    Some(affinity) => affinity,
                };

                let new_coordinate = match local_affinity {
                    Affinity::Upstream => Point::new(rect.right + 1.0, coordinate.y),
                    Affinity::Downstream => Point::new(rect.left - 1.0, coordinate.y),
                };
                self.get_coordinate_outside_placeholder(new_coordinate, Some(local_affinity))
            }
        }
    }

    pub fn get_glyph_position_at_coordinate(&self, p: impl Into<Point>) -> PositionWithAffinity {
        let point: Point = p.into();
        let coordinate = self.get_coordinate_outside_placeholder(point, None);
        self.paragraph
            .borrow()
            .get_glyph_position_at_coordinate(coordinate)
    }

    pub fn get_placeholder_at_index(&self, index: usize) -> &PlaceholderStyle {
        trace!("[get_placeholder_at_index] {:?}", index);
        let placeholders: Vec<&PlaceholderStyle> =
            (self.text.pieces.iter().map(|piece| match piece {
                ParagraphPiece::Text(_) => None,
                ParagraphPiece::Placeholder(placeholder, _) => Some(placeholder),
            }))
            .filter(|piece| piece.is_some())
            .map(|placeholder| placeholder.unwrap())
            .collect();

        assert!(
            index < placeholders.len(),
            "placeholder index ({:?}) must be less then placeholder count ({:?})",
            index,
            placeholders.len()
        );
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

        trace!("[get_placeholder_at_coordinate] {:?}", point);

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
                trace!("[get_placeholder_at_coordinate] Found placeholder at {:?} index {:?} with affinity {:?}", point, placeholder_index, affinity);
                return result;
            }
            placeholder_index = placeholder_index + 1;
        }

        trace!(
            "[get_placeholder_at_coordinate] No placeholder at {:?}",
            point
        );
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

    pub fn get_line_metrics<'a>(&self, paragraph: &'a Paragraph) -> Vec<LineMetrics<'a>> {
        paragraph.get_line_metrics()
    }

    pub fn line_number(&self) -> usize {
        self.paragraph.borrow().line_number()
    }

    pub fn get_line_ranges(&self) -> Vec<Range<usize>> {
        let paragraph = self.paragraph.borrow();
        let line_metrics = self.get_line_metrics(&paragraph);
        line_metrics
            .iter()
            .map(|each_metric| each_metric.start_index..each_metric.end_index)
            .collect()
    }

    pub fn max_width(&self) -> scalar {
        self.paragraph.borrow().max_width()
    }

    pub fn height(&self) -> scalar {
        self.paragraph.borrow().height()
    }

    pub fn longest_line(&self) -> scalar {
        self.paragraph.borrow().longest_line()
    }

    pub fn char_count(&self) -> usize {
        self.text.char_count()
    }

    pub fn get_rects_for_char_range(
        &self,
        char_range: Range<usize>,
        rect_height_style: RectHeightStyle,
        rect_width_style: RectWidthStyle,
    ) -> Vec<TextBox> {
        let glyph_range = self.text.get_glyph_range_for_char_range(char_range.clone());
        self.get_rects_for_range(glyph_range, rect_height_style, rect_width_style)
    }

    pub fn get_line_index_for_glyph_offset(&self, offset: PositionWithAffinity) -> usize {
        let paragraph = self.paragraph.borrow();
        let line_metrics = self.get_line_metrics(&paragraph);

        let glyph_offset = offset.position;
        if glyph_offset < 0 {
            return 0;
        }
        let glyph_offset = match TryInto::<usize>::try_into(glyph_offset) {
            Ok(offset) => offset,
            Err(error) => {
                error!(
                    "Failed to cast {} from {} to {}: {}",
                    glyph_offset,
                    type_name::<isize>(),
                    type_name::<usize>(),
                    error
                );
                0
            }
        };

        let affinity = offset.affinity;
        for (index, line) in line_metrics.iter().enumerate() {
            // line's end_index is exclusive, therefore we should use `less than`
            if glyph_offset < line.end_index {
                return index;
            }
            if glyph_offset == line.end_index && affinity == Affinity::Upstream {
                return index;
            }
        }
        self.line_number()
    }

    pub fn get_line_index_for_char_offset(&self, offset: usize) -> usize {
        let glyph_offset = self.text.get_glyph_offset_for_char_offset(offset);

        let paragraph = self.paragraph.borrow();
        let line_metrics = self.get_line_metrics(&paragraph);
        for (index, line) in line_metrics.iter().enumerate() {
            // line's end_index is exclusive, therefore we should use `less than`
            if glyph_offset < line.end_index {
                return index;
            }
        }
        self.line_number()
    }

    pub fn get_line_index_at_coordinate(&self, p: impl Into<Point>) -> usize {
        let position_with_affinity = self.get_glyph_position_at_coordinate(p);
        if position_with_affinity.position < 0 {
            return 0;
        }
        self.get_line_index_for_glyph_offset(position_with_affinity)
    }

    pub fn get_line_height(&self, index: usize) -> scalar {
        if self.line_number() == 0 && index == 0 {
            return self.height();
        }

        let paragraph = self.paragraph.borrow();
        let line_metrics = self.get_line_metrics(&paragraph);
        line_metrics.as_slice()[index].height as scalar
    }

    pub fn get_line_width(&self, index: usize) -> scalar {
        if self.line_number() == 0 && index == 0 {
            return 0.0;
        }

        let paragraph = self.paragraph.borrow();
        let line_metrics = self.get_line_metrics(&paragraph);
        line_metrics.as_slice()[index].width as scalar
    }

    pub fn get_line_end_character_index(&self, index: usize) -> usize {
        if self.line_number() == 0 && index == 0 {
            return 0;
        }
        if self.line_number() == (index + 1) {
            return self.text.char_count();
        }
        let paragraph = self.paragraph.borrow();
        let line_metrics = self.get_line_metrics(&paragraph);
        let end_glyph_index = (line_metrics.as_slice()[index].end_index - 1).max(0);
        self.text.get_char_offset_for_glyph_offset(end_glyph_index)
    }
}

#[no_mangle]
pub fn skia_paragraph_layout(paragraph_ptr: *mut ValueBox<ParagraphWithText>, width: scalar) {
    paragraph_ptr
        .with_mut_ok(|paragraph| {
            paragraph.layout(width);
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_paint(
    paragraph_ptr: *mut ValueBox<ParagraphWithText>,
    canvas_ptr: *mut ReferenceBox<Canvas>,
    x: scalar,
    y: scalar,
) {
    paragraph_ptr
        .with_ref_ok(|paragraph| {
            canvas_ptr.with_not_null(|canvas| {
                paragraph.paint(canvas, Point::new(x, y));
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_get_height(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> scalar {
    paragraph_ptr
        .with_ref_ok(|paragraph| paragraph.height())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_get_max_width(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> scalar {
    paragraph_ptr
        .with_ref_ok(|paragraph| paragraph.max_width())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_get_longest_line(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> scalar {
    paragraph_ptr
        .with_ref_ok(|paragraph| paragraph.longest_line())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_get_line_number(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> usize {
    paragraph_ptr
        .with_ref_ok(|paragraph| paragraph.line_number())
        .or_log(0)
}

#[no_mangle]
pub fn skia_paragraph_get_char_count(paragraph_ptr: *mut ValueBox<ParagraphWithText>) -> usize {
    paragraph_ptr
        .with_ref_ok(|paragraph| paragraph.char_count())
        .or_log(0)
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_placeholders(
    paragraph: *mut ValueBox<ParagraphWithText>,
) -> *mut ValueBox<ArrayBox<f32>> {
    paragraph
        .with_ref_ok(|paragraph| {
            let rectangles = paragraph.get_rects_for_placeholders();
            let mut points: Vec<f32> = Vec::with_capacity(rectangles.len() * 4);
            for text_box in rectangles {
                points.push(text_box.rect.left);
                points.push(text_box.rect.top);
                points.push(text_box.rect.right);
                points.push(text_box.rect.bottom);
            }
            ValueBox::new(ArrayBox::from_vector(points))
        })
        .into_raw()
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_glyph_range(
    paragraph: *mut ValueBox<ParagraphWithText>,
    start: usize,
    end: usize,
    rect_height_style: RectHeightStyle,
    rect_width_style: RectWidthStyle,
) -> *mut ValueBox<ArrayBox<f32>> {
    paragraph
        .with_ref_ok(|paragraph| {
            let rectangles =
                paragraph.get_rects_for_range(start..end, rect_height_style, rect_width_style);
            let mut points: Vec<f32> = Vec::with_capacity(rectangles.len() * 4);
            for text_box in rectangles {
                points.push(text_box.rect.left);
                points.push(text_box.rect.top);
                points.push(text_box.rect.right);
                points.push(text_box.rect.bottom);
            }
            ValueBox::new(ArrayBox::from_vector(points))
        })
        .into_raw()
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_char_range(
    paragraph: *mut ValueBox<ParagraphWithText>,
    start: usize,
    end: usize,
    rect_height_style: RectHeightStyle,
    rect_width_style: RectWidthStyle,
) -> *mut ValueBox<ArrayBox<f32>> {
    paragraph
        .with_ref_ok(|paragraph| {
            let rectangles =
                paragraph.get_rects_for_char_range(start..end, rect_height_style, rect_width_style);
            let mut points: Vec<f32> = Vec::with_capacity(rectangles.len() * 4);
            for text_box in rectangles {
                points.push(text_box.rect.left);
                points.push(text_box.rect.top);
                points.push(text_box.rect.right);
                points.push(text_box.rect.bottom);
            }
            ValueBox::new(ArrayBox::from_vector(points))
        })
        .into_raw()
}

#[no_mangle]
pub fn skia_paragraph_print(paragraph_ptr: *mut ValueBox<ParagraphWithText>) {
    paragraph_ptr.with_not_null(|paragraph_with_text| {
        let paragraph = paragraph_with_text.paragraph.borrow();
        let line_metrics = paragraph_with_text.get_line_metrics(&paragraph);
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
pub fn skia_paragraph_get_glyph_offset_for_char_offset(
    paragraph: *mut ValueBox<ParagraphWithText>,
    offset: usize,
) -> usize {
    paragraph
        .with_ref_ok(|paragraph| paragraph.text.get_glyph_offset_for_char_offset(offset))
        .or_log(0)
}

#[no_mangle]
pub fn skia_paragraph_get_line_height(
    paragraph: *mut ValueBox<ParagraphWithText>,
    index: usize,
) -> scalar {
    paragraph
        .with_ref_ok(|paragraph| paragraph.get_line_height(index))
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_get_line_width(
    paragraph: *mut ValueBox<ParagraphWithText>,
    index: usize,
) -> scalar {
    paragraph
        .with_ref_ok(|paragraph| paragraph.get_line_width(index))
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_paragraph_get_line_end_character_index(
    paragraph: *mut ValueBox<ParagraphWithText>,
    index: usize,
) -> usize {
    paragraph
        .with_ref_ok(|paragraph| paragraph.get_line_end_character_index(index))
        .or_log(0)
}

#[no_mangle]
pub fn skia_paragraph_get_line_index_for_char(
    paragraph: *mut ValueBox<ParagraphWithText>,
    index: usize,
) -> usize {
    paragraph
        .with_ref_ok(|paragraph| paragraph.get_line_index_for_char_offset(index))
        .or_log(0)
}

#[no_mangle]
pub fn skia_paragraph_get_line_index_at_coordinate(
    paragraph: *mut ValueBox<ParagraphWithText>,
    x: scalar,
    y: scalar,
) -> usize {
    paragraph
        .with_ref_ok(|paragraph| paragraph.get_line_index_at_coordinate(Point::new(x, y)))
        .or_log(0)
}

#[no_mangle]
pub fn skia_paragraph_drop(ptr: *mut ValueBox<ParagraphWithText>) {
    ptr.release();
}
