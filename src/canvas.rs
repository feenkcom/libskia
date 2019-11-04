use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use skia_safe::{Canvas, BlendMode, Color, Paint, Point, scalar, Rect, Path, ClipOp, IRect, QuickReject, Matrix, Vector, TextBlob};
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
pub fn skia_canvas_draw_point(canvas_ptr: *mut ReferenceBox<Canvas>, x: scalar, y: scalar, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_point(Point::new(x, y), paint);
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
pub fn skia_canvas_draw_oval(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_oval(Rect::new(left, top, right, bottom), paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_circle(canvas_ptr: *mut ReferenceBox<Canvas>, center_x: scalar, center_y: scalar, radius: scalar, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_circle(Point::new(center_x,center_y), radius, paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rounded_rectangle(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar, rx: scalar, ry: scalar, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_round_rect(Rect::new(left, top, right, bottom), rx, ry, paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_path(canvas_ptr: *mut ReferenceBox<Canvas>, path_ptr: *mut ValueBox<Path>, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            path_ptr.with(|path| {
                canvas.draw_path(path, paint);
            })
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_text_blob(canvas_ptr: *mut ReferenceBox<Canvas>, text_blob_ptr: *mut ValueBox<TextBlob>, x: scalar, y: scalar, paint_ptr: *mut ValueBox<Paint>) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            text_blob_ptr.with(|text_blob| {
                canvas.draw_text_blob(text_blob, Point::new(x, y), paint);
            })
        });
    });
}

#[no_mangle]
pub fn skia_canvas_clip_rect(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar, clip_op: ClipOp, do_anti_alias: bool) {
    canvas_ptr.with(|canvas| {
        canvas.clip_rect(Rect::new(left, top, right, bottom), clip_op, do_anti_alias);
    });
}

#[no_mangle]
pub fn skia_canvas_clip_path(canvas_ptr: *mut ReferenceBox<Canvas>, path_ptr: *mut ValueBox<Path>, clip_op: ClipOp, do_anti_alias: bool) {
    canvas_ptr.with(|canvas| {
        path_ptr.with(|path| {
            canvas.clip_path(path, clip_op, do_anti_alias);
        })
    });
}

#[no_mangle]
pub fn skia_canvas_local_clip_bounds(canvas_ptr: *mut ReferenceBox<Canvas>, rect_ptr: *mut ValueBox<Rect>) {
    canvas_ptr.with(|canvas| {
        rect_ptr.with(|rectangle| {
            match canvas.local_clip_bounds() {
                None => {},
                Some(local_bounds) => { rectangle.set_ltrb(local_bounds.left, local_bounds.top, local_bounds.right, local_bounds.bottom) },
            }
        })
    });
}

#[no_mangle]
pub fn skia_canvas_device_clip_bounds(canvas_ptr: *mut ReferenceBox<Canvas>, rect_ptr: *mut ValueBox<IRect>) {
    canvas_ptr.with(|canvas| {
        rect_ptr.with(|rectangle| {
            match canvas.device_clip_bounds() {
                None => {},
                Some(device_bounds) => { rectangle.set_ltrb(device_bounds.left, device_bounds.top, device_bounds.right, device_bounds.bottom) },
            }
        })
    });
}

#[no_mangle]
pub fn skia_canvas_quick_reject_rectangle(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar) -> bool {
    canvas_ptr.with(|canvas| canvas.quick_reject(&Rect::new(left, top, right, bottom)))
}

#[no_mangle]
pub fn skia_canvas_quick_reject_path(canvas_ptr: *mut ReferenceBox<Canvas>, path_ptr: *mut ValueBox<Path>) -> bool {
    canvas_ptr.with(|canvas|
        path_ptr.with(|path| canvas.quick_reject(path.as_ref())))
}

#[no_mangle]
pub fn skia_canvas_translate(canvas_ptr: *mut ReferenceBox<Canvas>, x: scalar, y: scalar) {
    canvas_ptr.with(|canvas| { canvas.translate(Vector::new(x, y)); });
}

#[no_mangle]
pub fn skia_canvas_scale(canvas_ptr: *mut ReferenceBox<Canvas>, sx: scalar, sy: scalar) {
    canvas_ptr.with(|canvas| { canvas.scale((sx, sy)); });
}

#[no_mangle]
pub fn skia_canvas_rotate(canvas_ptr: *mut ReferenceBox<Canvas>, degrees: scalar, x: scalar, y: scalar) {
    canvas_ptr.with(|canvas| { canvas.rotate(degrees, Some(Point::new(x, y))); });
}

#[no_mangle]
pub fn skia_canvas_skew(canvas_ptr: *mut ReferenceBox<Canvas>, sx: scalar, sy: scalar) {
    canvas_ptr.with(|canvas| { canvas.skew((sx, sy)); });
}

#[no_mangle]
pub fn skia_canvas_concat_matrix(canvas_ptr: *mut ReferenceBox<Canvas>, matrix_ptr: *mut ValueBox<Matrix>) {
    canvas_ptr.with(|canvas|
        matrix_ptr.with(|matrix| {
            canvas.concat(matrix);
        }));
}

#[no_mangle]
pub fn skia_canvas_set_matrix(canvas_ptr: *mut ReferenceBox<Canvas>, matrix_ptr: *mut ValueBox<Matrix>) {
    canvas_ptr.with(|canvas|
        matrix_ptr.with(|matrix| {
            canvas.set_matrix(matrix);
        }));
}

#[no_mangle]
pub fn skia_canvas_get_matrix(canvas_ptr: *mut ReferenceBox<Canvas>, matrix_ptr: *mut ValueBox<Matrix>) {
    canvas_ptr.with(|canvas|
        matrix_ptr.with(|matrix| {
            let m = canvas.total_matrix();
            let mut buffer: [scalar; 9] = [0.0; 9];
            m.get_9(&mut buffer);
            matrix.set_9(&buffer);
        }));
}

#[no_mangle]
pub fn skia_canvas_reset_matrix(canvas_ptr: *mut ReferenceBox<Canvas>) {
    canvas_ptr.with(|canvas| { canvas.reset_matrix(); })
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