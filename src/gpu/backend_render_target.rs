use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::gpu::BackendRenderTarget;

#[no_mangle]
pub fn skia_backend_render_target_new_gl(
    width: i32,
    height: i32,
    sample_count: usize,
    stencil_bits: usize,
    fboid: std::os::raw::c_uint,
    format: std::os::raw::c_uint,
) -> *mut ValueBox<BackendRenderTarget> {
    let render_target = BackendRenderTarget::new_gl(
        (width, height),
        Some(sample_count),
        stencil_bits,
        FramebufferInfo { fboid, format },
    );

    ValueBox::new(render_target).into_raw()
}

#[no_mangle]
pub fn skia_backend_render_target_is_valid(_ptr: *mut ValueBox<BackendRenderTarget>) -> bool {
    _ptr.with_not_null_return(false, |backend_render_target| {
        backend_render_target.is_valid()
    })
}

#[no_mangle]
pub fn skia_backend_render_target_is_protected(_ptr: *mut ValueBox<BackendRenderTarget>) -> bool {
    _ptr.with_not_null_return(false, |backend_render_target| {
        backend_render_target.is_protected()
    })
}

#[no_mangle]
pub fn skia_backend_render_target_drop(_ptr: *mut ValueBox<BackendRenderTarget>) {
    _ptr.drop()
}
