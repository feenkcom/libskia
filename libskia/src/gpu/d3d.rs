use value_box::ValueBox;
use windows::Win32::Foundation::HWND;

use crate::gpu::platform_compositor::{PlatformCompositor, PlatformContext};

#[no_mangle]
pub fn skia_d3d_compositor_new_size(
    window: HWND,
    width: u32,
    height: u32,
) -> *mut ValueBox<PlatformCompositor> {
    ValueBox::new(PlatformCompositor::new(PlatformContext::D3D(
        compositor_skia_platform::D3D12Context::new(window, width, height),
    )))
    .into_raw()
}
