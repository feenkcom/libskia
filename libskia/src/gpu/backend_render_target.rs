use skia_safe::gpu::BackendRenderTarget;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[cfg(feature = "gl")]
#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_render_target_new_gl(
    width: i32,
    height: i32,
    sample_count: usize,
    stencil_bits: usize,
    fboid: std::os::raw::c_uint,
    format: std::os::raw::c_uint,
) -> OwnedPtr<BackendRenderTarget> {
    let render_target = skia_safe::gpu::backend_render_targets::make_gl(
        (width, height),
        Some(sample_count),
        stencil_bits,
        skia_safe::gpu::gl::FramebufferInfo {
            fboid,
            format,
            protected: skia_safe::gpu::Protected::No,
        },
    );

    OwnedPtr::new(render_target)
}

#[cfg(feature = "metal")]
#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_render_target_new_metal(
    width: i32,
    height: i32,
) -> OwnedPtr<BackendRenderTarget> {
    let texture_info = unsafe { skia_safe::gpu::mtl::TextureInfo::new(std::ptr::null_mut()) };

    let render_target =
        skia_safe::gpu::backend_render_targets::make_mtl((width, height), &texture_info);

    OwnedPtr::new(render_target)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_render_target_is_valid(
    backend_render_target: BorrowedPtr<BackendRenderTarget>,
) -> bool {
    backend_render_target.with_ref_ok(|_| true).or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_render_target_is_protected(
    backend_render_target: BorrowedPtr<BackendRenderTarget>,
) -> bool {
    backend_render_target
        .with_ref_ok(|backend_render_target| backend_render_target.is_protected())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_backend_render_target_drop(
    backend_render_target: OwnedPtr<BackendRenderTarget>,
) {
    drop(backend_render_target);
}
