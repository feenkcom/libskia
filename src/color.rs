use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::Color;

#[no_mangle]
pub fn skia_color_default() -> *mut ValueBox<Color> {
    ValueBox::new(Color::default()).into_raw()
}

#[no_mangle]
pub fn skia_color_get_red(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.r())
}

#[no_mangle]
pub fn skia_color_get_green(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.g())
}

#[no_mangle]
pub fn skia_color_get_blue(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.b())
}

#[no_mangle]
pub fn skia_color_get_alpha(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.a())
}

#[no_mangle]
pub fn skia_color_drop(_ptr: *mut ValueBox<Color>) {
    _ptr.drop();
}