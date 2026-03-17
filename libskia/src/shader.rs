use skia_safe::shaders::empty;
use skia_safe::Shader;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_shader_create_empty() -> OwnedPtr<Shader> {
    OwnedPtr::new(empty())
}

#[no_mangle]
pub fn skia_shader_is_opaque(shader_ptr: BorrowedPtr<Shader>) -> bool {
    shader_ptr
        .with_ref_ok(|shader| shader.is_opaque())
        .or_log(false)
}

#[no_mangle]
pub fn skia_shader_is_a_image(shader_ptr: BorrowedPtr<Shader>) -> bool {
    shader_ptr
        .with_ref_ok(|shader| shader.is_a_image())
        .or_log(false)
}

#[no_mangle]
pub fn skia_shader_drop(ptr: OwnedPtr<Shader>) {
    drop(ptr);
}
