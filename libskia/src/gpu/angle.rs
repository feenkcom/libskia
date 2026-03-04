use std::ffi::c_void;

use value_box::{ValueBox, ValueBoxIntoRaw};

use crate::gpu::{PlatformCompositor, PlatformContext};

#[no_mangle]
pub fn skia_angle_compositor_new_size(
    window: *mut c_void,
    width: u32,
    height: u32,
) -> *mut ValueBox<PlatformCompositor> {
    compositor_skia_platform::AngleContext::new(window, width as i32, height as i32)
        .map(|context| ValueBox::new(PlatformCompositor::new(PlatformContext::Angle(context))))
        .map_err(|error| error.into())
        .into_raw()
}
