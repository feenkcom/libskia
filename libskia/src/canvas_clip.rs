use float_cmp::{ApproxEq, F32Margin};
use skia_safe::{Canvas, ClipOp, IRect, Path, QuickReject, RRect, Rect, Vector, scalar};
use value_box::{BorrowedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_clip_rect(
    canvas: BorrowedPtr<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
    clip_op: ClipOp,
    do_anti_alias: bool,
) {
    canvas
        .with_ref_ok(|canvas| {
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
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_clip_path(
    canvas: BorrowedPtr<Canvas>,
    path: BorrowedPtr<Path>,
    offset_x: scalar,
    offset_y: scalar,
    clip_op: ClipOp,
    do_anti_alias: bool,
) {
    path.with_ref_ok(|path| {
        canvas
            .with_ref_ok(|canvas| {
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
            .log();
    })
    .log();
}

/// I clip the canvas with a rounded rectangle using Intersect operation and anti-alias
#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_clip_rounded_rectangle(
    canvas: BorrowedPtr<Canvas>,
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
    canvas
        .with_ref_ok(|canvas| {
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
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_clip_circle(
    canvas: BorrowedPtr<Canvas>,
    center_x: scalar,
    center_y: scalar,
    radius: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    canvas
        .with_ref_ok(|canvas| {
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
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_clip_oval(
    canvas: BorrowedPtr<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    offset_x: scalar,
    offset_y: scalar,
) {
    canvas
        .with_ref_ok(|canvas| {
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
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_local_clip_bounds(
    canvas: BorrowedPtr<Canvas>,
    mut rect: BorrowedPtr<Rect>,
) {
    rect.with_mut_ok(|rect| {
        canvas
            .with_ref_ok(|canvas| match canvas.local_clip_bounds() {
                None => {}
                Some(local_bounds) => rect.set_ltrb(
                    local_bounds.left,
                    local_bounds.top,
                    local_bounds.right,
                    local_bounds.bottom,
                ),
            })
            .log();
    })
    .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_device_clip_bounds(
    canvas: BorrowedPtr<Canvas>,
    mut rect: BorrowedPtr<IRect>,
) {
    rect.with_mut_ok(|rect| {
        canvas
            .with_ref_ok(|canvas| match canvas.device_clip_bounds() {
                None => {}
                Some(device_bounds) => rect.set_ltrb(
                    device_bounds.left,
                    device_bounds.top,
                    device_bounds.right,
                    device_bounds.bottom,
                ),
            })
            .log()
    })
    .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_quick_reject_rectangle(
    canvas: BorrowedPtr<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) -> bool {
    canvas
        .with_ref_ok(|canvas| canvas.quick_reject(&Rect::new(left, top, right, bottom)))
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_quick_reject_path(
    canvas: BorrowedPtr<Canvas>,
    path: BorrowedPtr<Path>,
) -> bool {
    path.with_ref_ok(|path| {
        canvas
            .with_ref_ok(|canvas| canvas.quick_reject(path.as_ref()))
            .or_log(false)
    })
    .or_log(false)
}
