use boxer::boxes::{ReferenceBox, ValueBox, ReferenceBoxPointer, ValueBoxPointer};
use skia_safe::{Canvas, scalar, Path, ClipOp, Rect, RRect, Vector, IRect, QuickReject};

#[no_mangle]
pub fn skia_canvas_clip_rect(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar, clip_op: ClipOp, do_anti_alias: bool) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clip_rect(Rect::new(left, top, right, bottom), clip_op, do_anti_alias);
    });
}

#[no_mangle]
pub fn skia_canvas_clip_path(canvas_ptr: *mut ReferenceBox<Canvas>, path_ptr: *mut ValueBox<Path>, clip_op: ClipOp, do_anti_alias: bool) {
    canvas_ptr.with_not_null(|canvas| {
        path_ptr.with_not_null(|path| {
            canvas.clip_path(path, clip_op, do_anti_alias);
        })
    });
}

/// I clip the canvas with a rounded rectangle using Intersect operation and anti-alias
#[no_mangle]
pub fn skia_canvas_clip_rounded_rectangle(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar, top: scalar, right: scalar, bottom: scalar,
    r_top_left: scalar, r_top_right: scalar, r_bottom_right: scalar, r_bottom_left: scalar) {
        canvas_ptr.with_not_null(|canvas| {
           canvas.clip_rrect(
               RRect::new_rect_radii(Rect::new(left, top, right, bottom),&[
                    Vector::new(r_top_left, r_top_left),
                    Vector::new(r_top_right, r_top_right),
                    Vector::new(r_bottom_right, r_bottom_right),
                    Vector::new(r_bottom_left, r_bottom_left)]),
           ClipOp::Intersect,
           true);
        });
}

#[no_mangle]
pub fn skia_canvas_clip_circle(canvas_ptr: *mut ReferenceBox<Canvas>, center_x: scalar, center_y: scalar, radius: scalar) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clip_rrect(
            RRect::new_oval(Rect::new(center_x - radius, center_y - radius, center_x + radius, center_y + radius)),
            ClipOp::Intersect,
            true);
    });
}

#[no_mangle]
pub fn skia_canvas_clip_oval(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar,) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clip_rrect(
            RRect::new_oval(Rect::new(left, top, right, bottom)),
            ClipOp::Intersect,
            true);
    });
}

#[no_mangle]
pub fn skia_canvas_local_clip_bounds(canvas_ptr: *mut ReferenceBox<Canvas>, rect_ptr: *mut ValueBox<Rect>) {
    canvas_ptr.with_not_null(|canvas| {
        rect_ptr.with_not_null(|rectangle| {
            match canvas.local_clip_bounds() {
                None => {},
                Some(local_bounds) => { rectangle.set_ltrb(local_bounds.left, local_bounds.top, local_bounds.right, local_bounds.bottom) },
            }
        })
    });
}

#[no_mangle]
pub fn skia_canvas_device_clip_bounds(canvas_ptr: *mut ReferenceBox<Canvas>, rect_ptr: *mut ValueBox<IRect>) {
    canvas_ptr.with_not_null(|canvas| {
        rect_ptr.with_not_null(|rectangle| {
            match canvas.device_clip_bounds() {
                None => {},
                Some(device_bounds) => { rectangle.set_ltrb(device_bounds.left, device_bounds.top, device_bounds.right, device_bounds.bottom) },
            }
        })
    });
}

#[no_mangle]
pub fn skia_canvas_quick_reject_rectangle(canvas_ptr: *mut ReferenceBox<Canvas>, left: scalar, top: scalar, right: scalar, bottom: scalar) -> bool {
    canvas_ptr.with_not_null_return(false, |canvas| canvas.quick_reject(&Rect::new(left, top, right, bottom)))
}

#[no_mangle]
pub fn skia_canvas_quick_reject_path(canvas_ptr: *mut ReferenceBox<Canvas>, path_ptr: *mut ValueBox<Path>) -> bool {
    canvas_ptr.with_not_null_return(false, |canvas|
        path_ptr.with_not_null_return(false, |path| canvas.quick_reject(path.as_ref())))
}