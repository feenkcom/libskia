use binary::context::Context;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::Surface;

#[no_mangle]
pub fn skia_binary_context_new(mut surface_ptr: *mut ValueBox<Surface>) -> *mut ValueBox<Context> {
    surface_ptr
        .with_value_consumed(|surface| ValueBox::new(Context::new_surface(surface)).into_raw())
}

#[no_mangle]
pub fn skia_binary_extract_surface(
    mut context_pointer: *mut ValueBox<Context>,
) -> *mut ValueBox<Surface> {
    context_pointer.with_value_consumed(|context| ValueBox::new(context.surface()).into_raw())
}

#[no_mangle]
pub fn skia_binary_execute(
    context_pointer: *mut ValueBox<Context>,
    data_ptr: *mut u8,
    length: usize,
) {
    context_pointer.with_not_null(|context| {
        let buffer = unsafe { std::slice::from_raw_parts(data_ptr, length) };
        context.execute(buffer);
    })
}

#[no_mangle]
pub fn skia_binary_context_drop(_ptr: *mut ValueBox<Context>) {
    _ptr.drop();
}
