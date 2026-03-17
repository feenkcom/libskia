use crate::value_box_compat::*;
use skia_safe::gpu::gl::Interface;
use std::ffi::c_void;
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr};

#[no_mangle]
pub fn skia_interface_new_native() -> OwnedPtr<Interface> {
    match Interface::new_native() {
        None => {
            if cfg!(debug_assertions) {
                eprintln!("[skia_context_new_gl] Unable to create native OpenGL interface");
            }
            std::ptr::null_mut()
        }
        Some(_interface) => OwnedPtr::new(_interface).into_raw(),
    }
}

#[cfg(feature = "gl")]
#[no_mangle]
pub fn skia_interface_new_load_with(
    callback: extern "C" fn(BorrowedPtr<StringBox>) -> *const c_void,
) -> OwnedPtr<Interface> {
    match Interface::new_load_with(|symbol| {
        let boxer_string = OwnedPtr::new(StringBox::from_string(symbol.to_string())).into_raw();
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
        Some(_interface) => OwnedPtr::new(_interface).into_raw(),
    }
}

#[no_mangle]
pub fn skia_interface_drop(mut ptr: OwnedPtr<Interface>) {
    ptr.release();
}
