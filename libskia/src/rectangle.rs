use skia_safe::{IRect, Rect, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

///
/// Rect
///

#[unsafe(no_mangle)]
pub fn skia_rectangle_f32_default() -> OwnedPtr<Rect> {
    OwnedPtr::new(Rect::default())
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_f32_set_ltrb(
    mut rectangle_ptr: BorrowedPtr<Rect>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) {
    rectangle_ptr
        .with_mut_ok(|rectangle| {
            rectangle.set_ltrb(left, top, right, bottom);
        })
        .log();
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_f32_left(rectangle_ptr: BorrowedPtr<Rect>) -> scalar {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.left())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_f32_top(rectangle_ptr: BorrowedPtr<Rect>) -> scalar {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.top())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_f32_right(rectangle_ptr: BorrowedPtr<Rect>) -> scalar {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.right())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_f32_bottom(rectangle_ptr: BorrowedPtr<Rect>) -> scalar {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.bottom())
        .or_log(0.0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_f32_drop(ptr: OwnedPtr<Rect>) {
    drop(ptr);
}

///
/// IRect
///

#[unsafe(no_mangle)]
pub fn skia_rectangle_i32_default() -> OwnedPtr<IRect> {
    OwnedPtr::new(IRect::default())
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_i32_set_ltrb(
    mut rectangle_ptr: BorrowedPtr<IRect>,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) {
    rectangle_ptr
        .with_mut_ok(|rectangle| {
            rectangle.set_ltrb(left, top, right, bottom);
        })
        .log();
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_i32_left(rectangle_ptr: BorrowedPtr<IRect>) -> i32 {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.left())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_i32_top(rectangle_ptr: BorrowedPtr<IRect>) -> i32 {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.top())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_i32_right(rectangle_ptr: BorrowedPtr<IRect>) -> i32 {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.right())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_i32_bottom(rectangle_ptr: BorrowedPtr<IRect>) -> i32 {
    rectangle_ptr
        .with_ref_ok(|rectangle| rectangle.bottom())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub fn skia_rectangle_i32_drop(ptr: OwnedPtr<IRect>) {
    drop(ptr);
}
