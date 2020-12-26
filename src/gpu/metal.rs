use boxer::ValueBox;
use skia_safe::gpu::Context;

#[no_mangle]
pub fn skia_context_new_metal(
    device_ptr: *mut std::ffi::c_void,
    queue: *mut std::ffi::c_void,
) -> *mut ValueBox<Context> {
    unsafe {
        match Context::new_metal(device_ptr, queue) {
            None => {
                if cfg!(debug_assertions) {
                    eprintln!("[skia_context_new_gl] Unable to create OpenGL context");
                }
                return std::ptr::null_mut();
            }
            Some(context) => ValueBox::new(context).into_raw(),
        }
    }
}
