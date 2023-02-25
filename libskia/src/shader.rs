use skia_safe::shaders::empty;
use skia_safe::Shader;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_shader_create_empty() -> *mut ValueBox<Shader> {
    ValueBox::new(empty()).into_raw()
}

#[no_mangle]
pub fn skia_shader_is_opaque(shader_ptr: *mut ValueBox<Shader>) -> bool {
    shader_ptr
        .with_ref_ok(|shader| shader.is_opaque())
        .or_log(false)
}

#[no_mangle]
pub fn skia_shader_is_a_image(shader_ptr: *mut ValueBox<Shader>) -> bool {
    shader_ptr
        .with_ref_ok(|shader| shader.is_a_image())
        .or_log(false)
}

#[no_mangle]
pub fn skia_shader_drop(ptr: *mut ValueBox<Shader>) {
    ptr.release();
}
