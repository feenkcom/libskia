use std::ffi::c_void;

use core_graphics_types::geometry::CGSize;
use objc2::runtime::AnyObject;
use value_box::OwnedPtr;

use crate::gpu::platform_compositor::{PlatformCompositor, PlatformContext};

#[unsafe(no_mangle)]
pub extern "C" fn skia_metal_compositor_new_size(
    ns_view: *mut c_void,
    width: u32,
    height: u32,
) -> OwnedPtr<PlatformCompositor> {
    let ns_view = ns_view.cast::<AnyObject>();
    OwnedPtr::new(PlatformCompositor::new(PlatformContext::Metal(
        compositor_skia_platform::MetalContext::new(
            ns_view as _,
            Some(CGSize::new(width.into(), height.into())),
        ),
    )))
}
