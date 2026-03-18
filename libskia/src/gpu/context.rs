use skia_safe::ColorType;
use skia_safe::gpu::DirectContext;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[cfg(feature = "gl")]
#[unsafe(no_mangle)]
pub extern "C" fn skia_context_new_gl(
    interface: BorrowedPtr<skia_safe::gpu::gl::Interface>,
) -> OwnedPtr<DirectContext> {
    interface
        .with_clone_ok(|interface| match skia_safe::gpu::direct_contexts::make_gl(interface, None) {
            None => {
                if cfg!(debug_assertions) {
                    eprintln!("[skia_context_new_gl] Unable to create OpenGL context");
                }
                OwnedPtr::null()
            }
            Some(context) => OwnedPtr::new(context),
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_context_get_max_texture_size(context: BorrowedPtr<DirectContext>) -> i32 {
    context
        .with_ref_ok(|context| context.max_texture_size())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_context_get_max_render_target_size(
    context: BorrowedPtr<DirectContext>,
) -> i32 {
    context
        .with_ref_ok(|context| context.max_render_target_size())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_context_get_max_surface_sample_count_for_color_type(
    context: BorrowedPtr<DirectContext>,
    color_type: ColorType,
) -> usize {
    context
        .with_ref_ok(|context| context.max_surface_sample_count_for_color_type(color_type))
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_context_is_color_type_supported_as_image(
    context: BorrowedPtr<DirectContext>,
    color_type: ColorType,
) -> bool {
    context
        .with_clone_ok(|context| context.color_type_supported_as_image(color_type))
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_context_is_color_type_supported_as_surface(
    context: BorrowedPtr<DirectContext>,
    color_type: ColorType,
) -> bool {
    context
        .with_clone_ok(|context| context.color_type_supported_as_surface(color_type))
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_context_flush(mut context: BorrowedPtr<DirectContext>) {
    context
        .with_mut_ok(|context| {
            context.flush_and_submit();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_context_drop(context: OwnedPtr<DirectContext>) {
    drop(context);
}
