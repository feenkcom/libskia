use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::BoxerString;
use boxer::CBox;
use boxer::{assert_box, function};
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
    callback: extern "C" fn(*mut BoxerString) -> *const c_void,
) -> *mut ValueBox<Interface> {
    match Interface::new_load_with(|symbol| {
        let boxer_string = CBox::into_raw(BoxerString::from_slice(symbol));
        let func_ptr = callback(boxer_string);
        CBox::drop(boxer_string);
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
pub fn skia_interface_drop(_ptr: *mut ValueBox<Interface>) {
    _ptr.drop()
}

#[no_mangle]
pub fn skia_context_new_gl(mut _interface_ptr: *mut ValueBox<Interface>) -> *mut ValueBox<Context> {
    assert_box(_interface_ptr, function!());

    if !_interface_ptr.is_valid() {
        return std::ptr::null_mut();
    }

    _interface_ptr.with_value_consumed(|interface| match Context::new_gl(Some(interface)) {
        None => {
            if cfg!(debug_assertions) {
                eprintln!("[skia_context_new_gl] Unable to create OpenGL context");
            }
            return std::ptr::null_mut();
        }
        Some(_context) => ValueBox::new(_context).into_raw(),
    })
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
