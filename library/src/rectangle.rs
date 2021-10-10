use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::{scalar, IRect, Rect};

///
/// Rect
///

#[no_mangle]
pub fn skia_rectangle_f32_default() -> *mut ValueBox<Rect> {
    ValueBox::new(Rect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rectangle_f32_set_ltrb(
    rectangle_ptr: *mut ValueBox<Rect>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) {
    rectangle_ptr.with_not_null(|rectangle| {
        rectangle.set_ltrb(left, top, right, bottom);
    });
}

#[no_mangle]
pub fn skia_rectangle_f32_left(rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    rectangle_ptr.with_not_null_return(0.0, |rectangle| rectangle.left())
}

#[no_mangle]
pub fn skia_rectangle_f32_top(rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    rectangle_ptr.with_not_null_return(0.0, |rectangle| rectangle.top())
}

#[no_mangle]
pub fn skia_rectangle_f32_right(rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    rectangle_ptr.with_not_null_return(0.0, |rectangle| rectangle.right())
}

#[no_mangle]
pub fn skia_rectangle_f32_bottom(rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    rectangle_ptr.with_not_null_return(0.0, |rectangle| rectangle.bottom())
}

#[no_mangle]
pub fn skia_rectangle_f32_drop(ptr: &mut *mut ValueBox<Rect>) {
    drop!(ptr);
}

///
/// IRect
///

#[no_mangle]
pub fn skia_rectangle_i32_default() -> *mut ValueBox<IRect> {
    ValueBox::new(IRect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rectangle_i32_set_ltrb(
    rectangle_ptr: *mut ValueBox<IRect>,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) {
    rectangle_ptr.with_not_null(|rectangle| {
        rectangle.set_ltrb(left, top, right, bottom);
    });
}

#[no_mangle]
pub fn skia_rectangle_i32_left(rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    rectangle_ptr.with_not_null_return(0, |rectangle| rectangle.left())
}

#[no_mangle]
pub fn skia_rectangle_i32_top(rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    rectangle_ptr.with_not_null_return(0, |rectangle| rectangle.top())
}

#[no_mangle]
pub fn skia_rectangle_i32_right(rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    rectangle_ptr.with_not_null_return(0, |rectangle| rectangle.right())
}

#[no_mangle]
pub fn skia_rectangle_i32_bottom(rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    rectangle_ptr.with_not_null_return(0, |rectangle| rectangle.bottom())
}

#[no_mangle]
pub fn skia_rectangle_i32_drop(ptr: &mut *mut ValueBox<IRect>) {
    drop!(ptr);
}
