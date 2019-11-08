extern crate boxer;
extern crate skia_safe;

use std::os::raw::c_void;

pub mod canvas;
pub mod color;
pub mod color_space;
pub mod enums;
pub mod gradient;
pub mod image;
pub mod image_info;
pub mod matrix;
pub mod paint;
pub mod path;
pub mod rectangle;
pub mod rounded_rectangle;
pub mod shader;
pub mod surface;
pub mod surface_props;
pub mod text;
pub mod types;


#[no_mangle]
pub fn skia_test() -> bool {
    true
}

#[no_mangle]
pub fn skia_null_ptr() -> *mut c_void {
    std::ptr::null_mut()
}