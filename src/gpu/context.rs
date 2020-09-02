use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::gpu::gl::Interface;
use skia_safe::gpu::Context;
use skia_safe::ColorType;
use std::os::raw::c_void;

#[no_mangle]
pub fn skia_interface_new_native() -> *mut ValueBox<Interface> {
    match Interface::new_native() {
        None => {
            if cfg!(debug_assertions) {
                eprintln!("[skia_context_new_gl] Unable to create native OpenGL interface");
            }
            std::ptr::null_mut()
        }
        Some(_interface) => ValueBox::new(_interface).into_raw(),
    }
}

#[no_mangle]
pub fn skia_interface_new_load_with(
    callback: extern "C" fn(*mut ValueBox<BoxerString>) -> *const c_void,
) -> *mut ValueBox<Interface> {
    match Interface::new_load_with(|symbol| {
        let mut boxer_string =
            ValueBox::new(BoxerString::from_string(symbol.to_string())).into_raw();
        let func_ptr = callback(boxer_string);
        boxer_string.drop();
        if cfg!(debug_assertions) {
            eprintln!(
                "[skia_interface_new_load_with] GL func: {:?}; address: {:?}",
                symbol, func_ptr
            );
        }
        func_ptr
    }) {
        None => {
            if cfg!(debug_assertions) {
                eprintln!("[skia_interface_new_load_with] Unable to load native OpenGL interface");
            }
            std::ptr::null_mut()
        }
        Some(_interface) => ValueBox::new(_interface).into_raw(),
    }
}

#[no_mangle]
pub fn skia_interface_drop(mut ptr: *mut ValueBox<Interface>) {
    ptr.drop()
}

#[no_mangle]
pub fn skia_context_new_gl(interface_ptr: *mut ValueBox<Interface>) -> *mut ValueBox<Context> {
    interface_ptr.with_not_null_value_return(
        std::ptr::null_mut(),
        |interface| match Context::new_gl(Some(interface)) {
            None => {
                if cfg!(debug_assertions) {
                    eprintln!("[skia_context_new_gl] Unable to create OpenGL context");
                }
                return std::ptr::null_mut();
            }
            Some(_context) => ValueBox::new(_context).into_raw(),
        },
    )
}

#[no_mangle]
pub fn skia_context_get_max_texture_size(context_ptr: *mut ValueBox<Context>) -> i32 {
    context_ptr.with_not_null_return(0, |context| context.max_texture_size())
}

#[no_mangle]
pub fn skia_context_get_max_render_target_size(context_ptr: *mut ValueBox<Context>) -> i32 {
    context_ptr.with_not_null_return(0, |context| context.max_render_target_size())
}

#[no_mangle]
pub fn skia_context_get_max_surface_sample_count_for_color_type(
    context_ptr: *mut ValueBox<Context>,
    color_type: ColorType,
) -> usize {
    context_ptr.with_not_null_return(0, |context| {
        context.max_surface_sample_count_for_color_type(color_type)
    })
}

#[no_mangle]
pub fn skia_context_is_color_type_supported_as_image(
    context_ptr: *mut ValueBox<Context>,
    color_type: ColorType,
) -> bool {
    context_ptr.with_not_null_return(false, |context| {
        context.color_type_supported_as_image(color_type)
    })
}

#[no_mangle]
pub fn skia_context_is_color_type_supported_as_surface(
    context_ptr: *mut ValueBox<Context>,
    color_type: ColorType,
) -> bool {
    context_ptr.with_not_null_return(false, |context| {
        context.color_type_supported_as_surface(color_type)
    })
}

#[no_mangle]
pub fn skia_context_flush(_ptr: *mut ValueBox<Context>) {
    _ptr.with_not_null(|context| {
        context.flush_and_submit();
    });
}

#[no_mangle]
pub fn skia_context_drop(ptr: &mut *mut ValueBox<Context>) {
    drop!(ptr);
}
