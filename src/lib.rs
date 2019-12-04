extern crate boxer;
extern crate float_cmp;
extern crate skia_safe;

use std::os::raw::c_void;

#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

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
