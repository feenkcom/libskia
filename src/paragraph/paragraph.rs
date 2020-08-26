use boxer::boxes::{ValueBox, ReferenceBox, ReferenceBoxPointer, ValueBoxPointer};
use skia_safe::textlayout::{Paragraph, RectWidthStyle, RectHeightStyle};
use skia_safe::{Canvas, scalar, Point};
use boxer::array::BoxerArrayPointF32;
use boxer::point::BoxerPointF32;

#[no_mangle]
pub fn skia_paragraph_layout(paragraph_ptr: *mut ValueBox<Paragraph>, width: scalar) {
    paragraph_ptr.with_not_null(|paragraph| {
        paragraph.layout(width);
    })
}

#[no_mangle]
pub fn skia_paragraph_paint(paragraph_ptr: *mut ValueBox<Paragraph>, canvas_ptr: *mut ReferenceBox<Canvas>, x: scalar, y: scalar) {
    paragraph_ptr.with_not_null(|paragraph| {
        canvas_ptr.with_not_null(|canvas| {
            paragraph.paint(canvas, Point::new(x, y));
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_get_height(paragraph_ptr: *mut ValueBox<Paragraph>) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |paragraph| {
        paragraph.height()
    })
}

#[no_mangle]
pub fn skia_paragraph_get_longest_line(paragraph_ptr: *mut ValueBox<Paragraph>) -> scalar {
    paragraph_ptr.with_not_null_return(0.0, |paragraph| {
        paragraph.longest_line()
    })
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_placeholders(paragraph_ptr: *mut ValueBox<Paragraph>) -> *mut ValueBox<BoxerArrayPointF32> {
    paragraph_ptr.with_not_null_return(std::ptr::null_mut(), |paragraph| {
        let mut points: Vec<BoxerPointF32> = vec![];
        for text_box in paragraph.get_rects_for_placeholders().iter() {
            points.push(BoxerPointF32::new(text_box.rect.x(), text_box.rect.y()));
            points.push(BoxerPointF32::new(text_box.rect.right(), text_box.rect.bottom()));
        }
        let mut array = BoxerArrayPointF32::new();
        array.set_vector(points);
        ValueBox::new(array).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_get_rects_for_range(paragraph_ptr: *mut ValueBox<Paragraph>, start: usize, end: usize, rect_height_style: RectHeightStyle, rect_width_style: RectWidthStyle) -> *mut ValueBox<BoxerArrayPointF32> {
    paragraph_ptr.with_not_null_return(std::ptr::null_mut(), |paragraph| {
        let mut points: Vec<BoxerPointF32> = vec![];
        for text_box in paragraph.get_rects_for_range(start..end, rect_height_style, rect_width_style).iter() {
            points.push(BoxerPointF32::new(text_box.rect.x(), text_box.rect.y()));
            points.push(BoxerPointF32::new(text_box.rect.right(), text_box.rect.bottom()));
        }
        let mut array = BoxerArrayPointF32::new();
        array.set_vector(points);
        ValueBox::new(array).into_raw()
    })
}

#[no_mangle]
pub fn skia_paragraph_print(paragraph_ptr: *mut ValueBox<Paragraph>) {
    paragraph_ptr.with_not_null(|paragraph| {
        let line_metrics = paragraph.get_line_metrics();
        for (line, lm) in line_metrics.iter().enumerate() {
            println!("line: {} width: {} end: {}", line + 1, lm.width, lm.end_index)
        }
    })
}

#[no_mangle]
pub fn skia_paragraph_drop(ptr: *mut ValueBox<Paragraph>) {
    ptr.drop();
}

#[test]
pub fn sparkle() {
    let mut sparkle = String::from("💖");

    let wstr = widestring::U32String::from_str(&sparkle);

    println!("{:?}", wstr);

    assert_eq!(sparkle.len(), 4);

    for char in sparkle.char_indices() {
        println!("{:?}", char);
    }
    println!("{:?}", sparkle.bytes());
}