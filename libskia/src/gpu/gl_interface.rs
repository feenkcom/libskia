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
            OwnedPtr::null()
        }
        Some(_interface) => OwnedPtr::new(_interface),
    }
}

#[cfg(feature = "gl")]
#[no_mangle]
pub fn skia_interface_new_load_with(
    callback: extern "C" fn(BorrowedPtr<StringBox>) -> *const c_void,
) -> OwnedPtr<Interface> {
    match Interface::new_load_with(|symbol| {
        let boxer_string = StringBox::from_string(symbol.to_string());
        let func_ptr = callback(BorrowedPtr::from_ref(&boxer_string));
        drop(boxer_string);
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
            OwnedPtr::null()
        }
        Some(_interface) => OwnedPtr::new(_interface),
    }
}

#[no_mangle]
pub fn skia_interface_drop(mut ptr: OwnedPtr<Interface>) {
    drop(ptr);
}
