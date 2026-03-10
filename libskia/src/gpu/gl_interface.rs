use std::ffi::c_void;
use skia_safe::gpu::gl::Interface;
use string_box::StringBox;
use value_box::{ValueBox, ValueBoxPointer};

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

#[cfg(feature = "gl")]
#[no_mangle]
pub fn skia_interface_new_load_with(
    callback: extern "C" fn(*mut ValueBox<StringBox>) -> *const c_void,
) -> *mut ValueBox<Interface> {
    match Interface::new_load_with(|symbol| {
        let boxer_string = ValueBox::new(StringBox::from_string(symbol.to_string())).into_raw();
        let func_ptr = callback(boxer_string);
        boxer_string.release();
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
pub fn skia_interface_drop(ptr: *mut ValueBox<Interface>) {
    ptr.release();
}