use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::Shader;
use skia_safe::shaders::empty;


#[no_mangle]
pub fn skia_shader_create_empty() -> *mut ValueBox<Shader> {
    ValueBox::new(empty()).into_raw()
}

#[no_mangle]
pub fn skia_shader_is_opaque(_shader_ptr: *mut ValueBox<Shader>) -> bool {
    _shader_ptr.with_not_null_return(false, |shader| shader.is_opaque() )
}

#[no_mangle]
pub fn skia_shader_is_a_image(_shader_ptr: *mut ValueBox<Shader>) -> bool {
    _shader_ptr.with_not_null_return(false, |shader| shader.is_a_image() )
}

#[no_mangle]
pub fn skia_shader_drop(_ptr: *mut ValueBox<Shader>) {
    _ptr.drop()
}