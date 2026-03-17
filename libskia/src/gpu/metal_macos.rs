use std::ffi::c_void;

use cocoa::base::id as cocoa_id;
use core_graphics_types::geometry::CGSize;
use value_box::OwnedPtr;

use crate::gpu::platform_compositor::{PlatformCompositor, PlatformContext};

#[no_mangle]
pub fn skia_metal_compositor_new_size(
    ns_view: *mut c_void,
    width: u32,
    height: u32,
) -> OwnedPtr<PlatformCompositor> {
    OwnedPtr::new(PlatformCompositor::new(PlatformContext::Metal(
        compositor_skia_platform::MetalContext::new(
            ns_view as cocoa_id,
            Some(CGSize::new(width.into(), height.into())),
        ),
    )))
}
