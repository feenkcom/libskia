#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
use std::os::raw::c_void;

// re-export the ffi methods
pub use compositor_ffi::*;
pub use compositor_skia_ffi::*;
pub use value_box_ffi::*;

pub mod canvas;
pub mod canvas_clip;
pub mod canvas_draw_fill;
pub mod canvas_draw_stroke;
pub mod canvas_optimized;
pub mod color;
pub mod color_space;
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
pub mod path_effect;
pub mod picture;
pub mod recorder;
pub mod rectangle;
pub mod rounded_rectangle;
pub mod shader;
pub mod surface;
pub mod surface_props;
#[cfg(feature = "svg")]
pub mod svg;
pub mod text;
pub mod types;

#[unsafe(no_mangle)]
pub extern "C" fn skia_test() -> bool {
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_icu_init() {
    warn!("no need to setup icu")
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_init_env_logger() {
    env_logger::init();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_null_ptr() -> *mut c_void {
    std::ptr::null_mut()
}
