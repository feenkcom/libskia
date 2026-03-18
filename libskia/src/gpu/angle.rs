use std::ffi::c_void;

use value_box::{BorrowedPtr, OwnedPtr};

use crate::gpu::{PlatformCompositor, PlatformContext};

#[unsafe(no_mangle)]
pub extern "C" fn skia_angle_compositor_new_size(
    window: *mut c_void,
    width: u32,
    height: u32,
) -> OwnedPtr<PlatformCompositor> {
    compositor_skia_platform::AngleContext::new(window, width as i32, height as i32, true)
        .map(|context| OwnedPtr::new(PlatformCompositor::new(PlatformContext::Angle(context))))
        .map_err(|error| error.into())
}
