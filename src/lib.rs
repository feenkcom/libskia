#[macro_use]
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
extern crate env_logger;

use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::{icu, Paint};
use std::os::raw::c_void;

#[macro_export]
macro_rules! drop {
    ($ptr:expr) => {
        trace!("{}", function!());
        $ptr.drop();
    };
}

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
pub fn skia_init_env_logger() {
    env_logger::init();

    debug!("Debug level logging enabled");
    info!("Info level logging enabled");
    error!("Error level logging enabled");
    warn!("Warning level logging enabled");
    trace!("Trace level logging enabled");
}

#[no_mangle]
pub fn skia_null_ptr() -> *mut c_void {
    std::ptr::null_mut()
}

#[no_mangle]
pub fn skia_value_box_is_valid(ptr: *mut ValueBox<Paint>) -> bool {
    println!("[skia_value_box_is_valid] paint_ptr: {:?}", ptr);
    println!("{}", ptr.has_value());
    ptr.has_value()
}
