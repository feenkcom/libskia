use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::BackendRenderTarget;

#[cfg(feature = "gl")]
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
        skia_safe::gpu::gl::FramebufferInfo { fboid, format },
    );

    ValueBox::new(render_target).into_raw()
}

#[cfg(feature = "metal")]
#[no_mangle]
pub fn skia_backend_render_target_new_metal(
    width: i32,
    height: i32,
    sample_count: usize,
) -> *mut ValueBox<BackendRenderTarget> {
    let texture_info = unsafe { skia_safe::gpu::mtl::TextureInfo::new(std::ptr::null_mut()) };

    let render_target =
        BackendRenderTarget::new_metal((width, height), sample_count as i32, &texture_info);

    ValueBox::new(render_target).into_raw()
}

#[no_mangle]
pub fn skia_backend_render_target_is_valid(
    backend_render_target_ptr: *mut ValueBox<BackendRenderTarget>,
) -> bool {
    backend_render_target_ptr.with_not_null_return(false, |backend_render_target| {
        backend_render_target.is_valid()
    })
}

#[no_mangle]
pub fn skia_backend_render_target_is_protected(
    backend_render_target_ptr: *mut ValueBox<BackendRenderTarget>,
) -> bool {
    backend_render_target_ptr.with_not_null_return(false, |backend_render_target| {
        backend_render_target.is_protected()
    })
}

#[no_mangle]
pub fn skia_backend_render_target_drop(ptr: *mut ValueBox<BackendRenderTarget>) {
    ptr.release();
}
