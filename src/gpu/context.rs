use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::gl::Interface;
use skia_safe::gpu::Context;
use skia_safe::ColorType;

#[no_mangle]
pub fn skia_context_new_gl() -> *mut ValueBox<Context> {
    let interface = match Interface::new_native() {
        None => {
            if cfg!(debug_assertions) {
                eprintln!("[skia_context_new_gl] Unable to create native OpenGL interface");
            }
            return std::ptr::null_mut();
        }
        Some(_interface) => _interface,
    };
    let context = match Context::new_gl(Some(interface)) {
        None => {
            if cfg!(debug_assertions) {
                eprintln!("[skia_context_new_gl] Unable to create OpenGL context");
            }
            return std::ptr::null_mut();
        }
        Some(_context) => _context,
    };

    ValueBox::new(context).into_raw()
}

#[no_mangle]
pub fn skia_context_get_max_texture_size(_ptr: *mut ValueBox<Context>) -> i32 {
    _ptr.with_not_null_return(0, |context| context.max_texture_size())
}

#[no_mangle]
pub fn skia_context_get_max_render_target_size(_ptr: *mut ValueBox<Context>) -> i32 {
    _ptr.with_not_null_return(0, |context| context.max_render_target_size())
}

#[no_mangle]
pub fn skia_context_get_max_surface_sample_count_for_color_type(
    _ptr: *mut ValueBox<Context>,
    color_type: ColorType,
) -> usize {
    _ptr.with_not_null_return(0, |context| {
        context.max_surface_sample_count_for_color_type(color_type)
    })
}

#[no_mangle]
pub fn skia_context_is_color_type_supported_as_image(
    _ptr: *mut ValueBox<Context>,
    color_type: ColorType,
) -> bool {
    _ptr.with_not_null_return(false, |context| {
        context.color_type_supported_as_image(color_type)
    })
}

#[no_mangle]
pub fn skia_context_is_color_type_supported_as_surface(
    _ptr: *mut ValueBox<Context>,
    color_type: ColorType,
) -> bool {
    _ptr.with_not_null_return(false, |context| {
        context.color_type_supported_as_surface(color_type)
    })
}

#[no_mangle]
pub fn skia_context_flush(_ptr: *mut ValueBox<Context>) {
    _ptr.with_not_null(|context| {
        context.flush();
    });
}

#[no_mangle]
pub fn skia_context_drop(_ptr: *mut ValueBox<Context>) {
    _ptr.drop()
}
