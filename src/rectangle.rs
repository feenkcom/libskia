use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{Rect, scalar};

#[no_mangle]
pub fn skia_rectangle_default() -> *mut ValueBox<Rect> {
    ValueBox::new(Rect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rectangle_set_ltrb(_rectangle_ptr: *mut ValueBox<Rect>, left: scalar, top: scalar, right: scalar, bottom: scalar) {
    _rectangle_ptr.with(|rectangle| { rectangle.set_ltrb(left, top, right, bottom); });
}

#[no_mangle]
pub fn skia_rectangle_drop(_ptr: *mut ValueBox<Rect>) {
    _ptr.drop();
}