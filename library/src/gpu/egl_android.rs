use crate::gpu::{PlatformCompositor, PlatformContext};
use value_box::ValueBox;

pub struct EglContext {}

#[no_mangle]
pub fn skia_egl_compositor_new_size(width: u32, height: u32) -> *mut ValueBox<PlatformCompositor> {
    ValueBox::new(PlatformCompositor::new(PlatformContext::Egl(EglContext {}))).into_raw()
}
