use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::{Canvas, BlendMode, Color};
use boxer::string::BoxerString;
use boxer::CBox;

#[no_mangle]
pub fn skia_blend_mode_to_string(_enum: BlendMode, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_blend_mode_to_int(_enum: BlendMode) -> i32 {
    unsafe { std::mem::transmute(_enum) }
}

#[no_mangle]
pub fn skia_canvas_draw_color(canvas_ptr: *mut ReferenceBox<Canvas>, r: u8, g: u8, b: u8, a: u8, blend_mode: BlendMode) {
    canvas_ptr.with(|canvas| { canvas.draw_color(Color::from_argb(a, r, g, b), blend_mode); });
}

#[no_mangle]
pub fn skia_canvas_drop(_ptr: *mut ReferenceBox<Canvas>) {
    _ptr.drop();
}