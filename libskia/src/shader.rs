use skia_safe::Shader;
use skia_safe::shaders::empty;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_shader_create_empty() -> OwnedPtr<Shader> {
    OwnedPtr::new(empty())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_shader_is_opaque(shader_ptr: BorrowedPtr<Shader>) -> bool {
    shader_ptr
        .with_ref_ok(|shader| shader.is_opaque())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_shader_is_a_image(shader_ptr: BorrowedPtr<Shader>) -> bool {
    shader_ptr
        .with_ref_ok(|shader| shader.is_a_image())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_shader_drop(ptr: OwnedPtr<Shader>) {
    drop(ptr);
}
