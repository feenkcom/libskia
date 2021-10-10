use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use boxer::{ValueBox, ValueBoxPointer};
use float_cmp::{ApproxEq, F32Margin};
use skia_safe::{scalar, Canvas, ClipOp, IRect, Path, QuickReject, RRect, Rect, Vector};

#[no_mangle]
pub fn skia_canvas_clip_rect(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
    clip_op: ClipOp,
    do_anti_alias: bool,
) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clip_rect(
            Rect::new(
                left + offset_x,
                top + offset_y,
                right + offset_x,
                bottom + offset_y,
            ),
            clip_op,
            do_anti_alias,
        );
    });
}

#[no_mangle]
pub fn skia_canvas_clip_path(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    path_ptr: *mut ValueBox<Path>,
    offset_x: scalar,
    offset_y: scalar,
    clip_op: ClipOp,
    do_anti_alias: bool,
) {
    canvas_ptr.with_not_null(|canvas| {
        path_ptr.with_not_null(|path| {
            if offset_x.approx_eq(0.0, F32Margin::default())
                && offset_y.approx_eq(0.0, F32Margin::default())
            {
                canvas.clip_path(path, clip_op, do_anti_alias);
            } else {
                canvas.clip_path(
                    &path.with_offset(Vector::new(offset_x, offset_y)),
                    clip_op,
                    do_anti_alias,
                );
            }
        })
    });
}

/// I clip the canvas with a rounded rectangle using Intersect operation and anti-alias
#[no_mangle]
pub fn skia_canvas_clip_rounded_rectangle(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_top_left: scalar,
    r_top_right: scalar,
    r_bottom_right: scalar,
    r_bottom_left: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clip_rrect(
            RRect::new_rect_radii(
                Rect::new(
                    left + offset_x,
                    top + offset_y,
                    right + offset_x,
                    bottom + offset_y,
                ),
                &[
                    Vector::new(r_top_left, r_top_left),
                    Vector::new(r_top_right, r_top_right),
                    Vector::new(r_bottom_right, r_bottom_right),
                    Vector::new(r_bottom_left, r_bottom_left),
                ],
            ),
            ClipOp::Intersect,
            true,
        );
    });
}

#[no_mangle]
pub fn skia_canvas_clip_circle(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    center_x: scalar,
    center_y: scalar,
    radius: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clip_rrect(
            RRect::new_oval(Rect::new(
                center_x + offset_x - radius,
                center_y + offset_y - radius,
                center_x + offset_x + radius,
                center_y + offset_y + radius,
            )),
            ClipOp::Intersect,
            true,
        );
    });
}

#[no_mangle]
pub fn skia_canvas_clip_oval(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clip_rrect(
            RRect::new_oval(Rect::new(
                left + offset_x,
                top + offset_y,
                right + offset_x,
                bottom + offset_y,
            )),
            ClipOp::Intersect,
            true,
        );
    });
}

#[no_mangle]
pub fn skia_canvas_local_clip_bounds(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    rect_ptr: *mut ValueBox<Rect>,
) {
    canvas_ptr.with_not_null(|canvas| {
        rect_ptr.with_not_null(|rectangle| match canvas.local_clip_bounds() {
            None => {}
            Some(local_bounds) => rectangle.set_ltrb(
                local_bounds.left,
                local_bounds.top,
                local_bounds.right,
                local_bounds.bottom,
            ),
        })
    });
}

#[no_mangle]
pub fn skia_canvas_device_clip_bounds(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    rect_ptr: *mut ValueBox<IRect>,
) {
    canvas_ptr.with_not_null(|canvas| {
        rect_ptr.with_not_null(|rectangle| match canvas.device_clip_bounds() {
            None => {}
            Some(device_bounds) => rectangle.set_ltrb(
                device_bounds.left,
                device_bounds.top,
                device_bounds.right,
                device_bounds.bottom,
            ),
        })
    });
}

#[no_mangle]
pub fn skia_canvas_quick_reject_rectangle(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) -> bool {
    canvas_ptr.with_not_null_return(false, |canvas| {
        canvas.quick_reject(&Rect::new(left, top, right, bottom))
    })
}

#[no_mangle]
pub fn skia_canvas_quick_reject_path(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    path_ptr: *mut ValueBox<Path>,
) -> bool {
    canvas_ptr.with_not_null_return(false, |canvas| {
        path_ptr.with_not_null_return(false, |path| canvas.quick_reject(path.as_ref()))
    })
}
