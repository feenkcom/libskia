use skia_safe::gpu;
use skia_safe::gpu::{BackendRenderTarget, Protected};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

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
    let render_target = gpu::backend_render_targets::make_gl(
        (width, height),
        Some(sample_count),
        stencil_bits,
        skia_safe::gpu::gl::FramebufferInfo {
            fboid,
            format,
            protected: Protected::No,
        },
    );

    ValueBox::new(render_target).into_raw()
}

#[cfg(feature = "metal")]
#[no_mangle]
pub fn skia_backend_render_target_new_metal(
    width: i32,
    height: i32,
) -> *mut ValueBox<BackendRenderTarget> {
    let texture_info = unsafe { skia_safe::gpu::mtl::TextureInfo::new(std::ptr::null_mut()) };

    let render_target = BackendRenderTarget::new_metal((width, height), &texture_info);

    ValueBox::new(render_target).into_raw()
}

#[no_mangle]
pub fn skia_backend_render_target_is_valid(
    backend_render_target_ptr: *mut ValueBox<BackendRenderTarget>,
) -> bool {
    backend_render_target_ptr
        .with_ref_ok(|_| true)
        .or_log(false)
}

#[no_mangle]
pub fn skia_backend_render_target_is_protected(
    backend_render_target_ptr: *mut ValueBox<BackendRenderTarget>,
) -> bool {
    backend_render_target_ptr
        .with_ref_ok(|backend_render_target| backend_render_target.is_protected())
        .or_log(false)
}

#[no_mangle]
pub fn skia_backend_render_target_drop(ptr: *mut ValueBox<BackendRenderTarget>) {
    ptr.release();
}
