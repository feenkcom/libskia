use boxer::boxes::{ReferenceBox, ValueBox, ReferenceBoxPointer, ValueBoxPointer};
use skia_safe::{Canvas, TextBlob, scalar, Point, Paint, Color, Rect};
use canvas::assert_canvas;

#[no_mangle]
pub fn skia_canvas_draw_rectangle_with_color(canvas_ptr: *mut ReferenceBox<Canvas>,
                                             left: scalar, top: scalar, right: scalar, bottom: scalar,
                                             r: u8, g: u8, b: u8, a: u8, antialias: bool) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.draw_rect(Rect::new(left, top, right, bottom), Paint::default().set_color(Color::from_argb(a, r, g, b)).set_anti_alias(antialias));
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rectangle_with_black_color(canvas_ptr: *mut ReferenceBox<Canvas>,
                                             left: scalar, top: scalar, right: scalar, bottom: scalar, antialias: bool) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.draw_rect(Rect::new(left, top, right, bottom), Paint::default().set_color(Color::BLACK).set_anti_alias(antialias));
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rectangle_with_white_color(canvas_ptr: *mut ReferenceBox<Canvas>,
                                             left: scalar, top: scalar, right: scalar, bottom: scalar, antialias: bool) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.draw_rect(Rect::new(left, top, right, bottom), Paint::default().set_color(Color::WHITE).set_anti_alias(antialias));
    });
}

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
    antialias: bool) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        text_blob_ptr.with_not_null(|text_blob| {
            canvas.draw_text_blob(text_blob, Point::new(x, y), Paint::default().set_color(Color::from_argb(a, r, g, b)).set_anti_alias(antialias));
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
    antialias: bool) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        text_blob_ptr.with_not_null(|text_blob| {
            canvas.draw_text_blob(text_blob, Point::new(x, y), Paint::default().set_color(Color::BLACK).set_anti_alias(antialias));
        });
    });
}