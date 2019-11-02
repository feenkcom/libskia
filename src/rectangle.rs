use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{Rect, scalar, IRect};

///
/// Rect
///

#[no_mangle]
pub fn skia_rectangle_f32_default() -> *mut ValueBox<Rect> {
    ValueBox::new(Rect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rectangle_set_f32_ltrb(_rectangle_ptr: *mut ValueBox<Rect>, left: scalar, top: scalar, right: scalar, bottom: scalar) {
    _rectangle_ptr.with(|rectangle| { rectangle.set_ltrb(left, top, right, bottom); });
}

#[no_mangle]
pub fn skia_rectangle_f32_left(_rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    _rectangle_ptr.with(|rectangle| rectangle.left())
}

#[no_mangle]
pub fn skia_rectangle_f32_top(_rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    _rectangle_ptr.with(|rectangle| rectangle.top())
}

#[no_mangle]
pub fn skia_rectangle_f32_right(_rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    _rectangle_ptr.with(|rectangle| rectangle.right())
}

#[no_mangle]
pub fn skia_rectangle_f32_bottom(_rectangle_ptr: *mut ValueBox<Rect>) -> scalar {
    _rectangle_ptr.with(|rectangle| rectangle.bottom())
}

#[no_mangle]
pub fn skia_rectangle_f32_drop(_ptr: *mut ValueBox<Rect>) {
    _ptr.drop();
}

///
/// IRect
///

#[no_mangle]
pub fn skia_rectangle_i32_default() -> *mut ValueBox<IRect> {
    ValueBox::new(IRect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rectangle_i32_set_ltrb(_rectangle_ptr: *mut ValueBox<IRect>, left: i32, top: i32, right: i32, bottom: i32) {
    _rectangle_ptr.with(|rectangle| { rectangle.set_ltrb(left, top, right, bottom); });
}

#[no_mangle]
pub fn skia_rectangle_i32_left(_rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    _rectangle_ptr.with(|rectangle| rectangle.left())
}

#[no_mangle]
pub fn skia_rectangle_i32_top(_rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    _rectangle_ptr.with(|rectangle| rectangle.top())
}

#[no_mangle]
pub fn skia_rectangle_i32_right(_rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    _rectangle_ptr.with(|rectangle| rectangle.right())
}

#[no_mangle]
pub fn skia_rectangle_i32_bottom(_rectangle_ptr: *mut ValueBox<IRect>) -> i32 {
    _rectangle_ptr.with(|rectangle| rectangle.bottom())
}

#[no_mangle]
pub fn skia_rectangle_i32_drop(_ptr: *mut ValueBox<IRect>) {
    _ptr.drop();
}