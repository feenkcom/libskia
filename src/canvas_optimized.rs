use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use canvas::assert_canvas;
use skia_safe::{scalar, Canvas, Color, Paint, Point, TextBlob};

/// Fill a given text blob with a color and disabled antialias
#[no_mangle]
pub fn skia_canvas_draw_text_blob_with_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    text_blob_ptr: *mut ValueBox<TextBlob>,
    x: scalar,
    y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    antialias: bool,
) {
    assert_canvas(canvas_ptr, function!());
    canvas_ptr.with_not_null(|canvas| {
        text_blob_ptr.with_not_null(|text_blob| {
            canvas.draw_text_blob(
                text_blob,
                Point::new(x, y),
                Paint::default()
                    .set_color(Color::from_argb(a, r, g, b))
                    .set_anti_alias(antialias),
            );
        });
    });
}

/// Fill a given text blob with a color and disabled antialias
#[no_mangle]
pub fn skia_canvas_draw_text_blob_with_black_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    text_blob_ptr: *mut ValueBox<TextBlob>,
    x: scalar,
    y: scalar,
    antialias: bool,
) {
    assert_canvas(canvas_ptr, function!());
    canvas_ptr.with_not_null(|canvas| {
        text_blob_ptr.with_not_null(|text_blob| {
            canvas.draw_text_blob(
                text_blob,
                Point::new(x, y),
                Paint::default()
                    .set_color(Color::BLACK)
                    .set_anti_alias(antialias),
            );
        });
    });
}
