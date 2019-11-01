use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use skia_safe::{Canvas, BlendMode, Color, Paint, Point, scalar, Rect};
use skia_safe::canvas::PointMode;
use boxer::array::{BoxerArray};

#[no_mangle]
pub fn skia_canvas_draw_color(canvas_ptr: *mut ReferenceBox<Canvas>, r: u8, g: u8, b: u8, a: u8, blend_mode: BlendMode) {
    canvas_ptr.with(|canvas| { canvas.draw_color(Color::from_argb(a, r, g, b), blend_mode); });
}

#[no_mangle]
pub fn skia_canvas_draw_paint(canvas_ptr: *mut ReferenceBox<Canvas>, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_paint(paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_points(canvas_ptr: *mut ReferenceBox<Canvas>, point_mode: PointMode, points_ptr: *mut ValueBox<BoxerArray<Point>>, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            points_ptr.with(|points| {
                canvas.draw_points(point_mode, points.to_slice(), paint);
            })
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_point(canvas_ptr: *mut ReferenceBox<Canvas>, point_ptr: *mut ValueBox<Point>, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            point_ptr.with_value(|point| {
                canvas.draw_point(point, paint);
            })
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_line(canvas_ptr: *mut ReferenceBox<Canvas>, from_x: scalar, from_y: scalar, to_x: scalar, to_y: scalar, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_line(Point::new(from_x, from_y), Point::new(to_x, to_y), paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rectangle(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_rect(Rect::new(left, top, right, bottom), paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_flush(canvas_ptr: *mut ReferenceBox<Canvas>) {
    canvas_ptr.with(|canvas| { canvas.flush(); })
}

#[no_mangle]
pub fn skia_canvas_save(canvas_ptr: *mut ReferenceBox<Canvas>) -> usize {
    canvas_ptr.with(|canvas| canvas.save())
}

#[no_mangle]
pub fn skia_canvas_save_count(canvas_ptr: *mut ReferenceBox<Canvas>) -> usize {
    canvas_ptr.with(|canvas| canvas.save_count())
}

#[no_mangle]
pub fn skia_canvas_restore(canvas_ptr: *mut ReferenceBox<Canvas>) {
    canvas_ptr.with(|canvas| { canvas.restore(); })
}

#[no_mangle]
pub fn skia_canvas_restore_to_count(canvas_ptr: *mut ReferenceBox<Canvas>, count: usize) {
    canvas_ptr.with(|canvas| { canvas.restore_to_count(count); })
}

#[no_mangle]
pub fn skia_canvas_drop(_ptr: *mut ReferenceBox<Canvas>) {
    _ptr.drop();
}