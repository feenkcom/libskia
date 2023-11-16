#![allow(incomplete_features)]
#![feature(specialization)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate phlow;
#[macro_use]
extern crate value_box;

use std::os::raw::c_void;

// re-export the ffi methods
pub use compositor_ffi::*;
pub use compositor_skia_ffi::*;
pub use phlow_extensions::CoreExtensions;
pub use phlow_ffi::*;
pub use value_box_ffi::*;

pub mod canvas;
pub mod canvas_clip;
pub mod canvas_draw_fill;
pub mod canvas_draw_stroke;
pub mod canvas_optimized;
pub mod color;
pub mod color_space;
pub mod enums;
mod extensions;
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
pub mod text;
pub mod types;
#[cfg(feature = "svg")]
pub mod svg;

define_extensions!(SkiaExtensions);
import_extensions!(SkiaExtensions, CoreExtensions);

#[no_mangle]
pub fn skia_test() -> bool {
    true
}

#[no_mangle]
pub fn skia_icu_init() {
    warn!("no need to setup icu")
}

#[no_mangle]
pub fn skia_init_env_logger() {
    env_logger::init();
}

#[no_mangle]
pub fn skia_null_ptr() -> *mut c_void {
    std::ptr::null_mut()
}
