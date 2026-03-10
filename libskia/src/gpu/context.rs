use skia_safe::gpu::DirectContext;
use skia_safe::ColorType;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[cfg(feature = "gl")]
#[no_mangle]
pub fn skia_context_new_gl(
    interface_ptr: *mut ValueBox<skia_safe::gpu::gl::Interface>,
) -> *mut ValueBox<DirectContext> {
    interface_ptr.with_not_null_value_return(std::ptr::null_mut(), |interface| {
        match DirectContext::new_gl(interface, None) {
            None => {
                if cfg!(debug_assertions) {
                    eprintln!("[skia_context_new_gl] Unable to create OpenGL context");
                }
                return std::ptr::null_mut();
            }
            Some(_context) => ValueBox::new(_context).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_context_get_max_texture_size(context: *mut ValueBox<DirectContext>) -> i32 {
    context
        .with_ref_ok(|context| context.max_texture_size())
        .or_log(0)
}

#[no_mangle]
pub fn skia_context_get_max_render_target_size(context: *mut ValueBox<DirectContext>) -> i32 {
    context
        .with_ref_ok(|context| context.max_render_target_size())
        .or_log(0)
}

#[no_mangle]
pub fn skia_context_get_max_surface_sample_count_for_color_type(
    context: *mut ValueBox<DirectContext>,
    color_type: ColorType,
) -> usize {
    context
        .with_ref_ok(|context| context.max_surface_sample_count_for_color_type(color_type))
        .or_log(0)
}

#[no_mangle]
pub fn skia_context_is_color_type_supported_as_image(
    context_ptr: *mut ValueBox<DirectContext>,
    color_type: ColorType,
) -> bool {
    context_ptr.with_not_null_return(false, |context| {
        context.color_type_supported_as_image(color_type)
    })
}

#[no_mangle]
pub fn skia_context_is_color_type_supported_as_surface(
    context_ptr: *mut ValueBox<DirectContext>,
    color_type: ColorType,
) -> bool {
    context_ptr.with_not_null_return(false, |context| {
        context.color_type_supported_as_surface(color_type)
    })
}

#[no_mangle]
pub fn skia_context_flush(ptr: *mut ValueBox<DirectContext>) {
    ptr.with_not_null(|context| {
        context.flush_and_submit();
    });
}

#[no_mangle]
pub fn skia_context_drop(ptr: *mut ValueBox<DirectContext>) {
    ptr.release();
}
