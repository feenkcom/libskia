extern crate boxer;
extern crate byteorder;
extern crate crossbeam;
extern crate float_cmp;
extern crate glutin;
extern crate num_cpus;
extern crate num_enum;
extern crate ordered_float;
extern crate skia_safe;
extern crate widestring;
#[macro_use]
extern crate log;

use skia_safe::icu;
use std::os::raw::c_void;

pub mod canvas;
pub mod canvas_clip;
pub mod canvas_draw_fill;
pub mod canvas_draw_stroke;
pub mod canvas_optimized;
pub mod color;
pub mod color_space;
pub mod compositor;
pub mod enums;
pub mod gpu;
pub mod gradient;
pub mod image;
pub mod image_filters;
pub mod image_info;
pub mod layer;
pub mod matrix;
pub mod paint;
pub mod paragraph;
pub mod path;
pub mod picture;
pub mod recorder;
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
pub fn skia_icu_init() {
    icu::init();
}

#[no_mangle]
pub fn skia_null_ptr() -> *mut c_void {
    std::ptr::null_mut()
}
