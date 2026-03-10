pub use platform_compositor::*;

#[cfg(target_os = "windows")]
pub use self::angle::*;
#[cfg(target_os = "windows")]
pub use self::d3d::*;
#[cfg(all(feature = "egl", target_os = "android"))]
pub use self::egl_android::*;
#[cfg(all(feature = "egl", feature = "wayland"))]
pub use self::egl_wayland::*;
#[cfg(feature = "x11")]
pub use self::gl_x11::*;
#[cfg(all(feature = "metal", target_os = "ios"))]
pub use self::metal_ios::*;
#[cfg(all(feature = "metal", target_os = "macos"))]
pub use self::metal_macos::*;

pub mod backend_render_target;
pub mod backend_texture;
pub mod context;
pub mod surface_gpu;

#[cfg(target_os = "macos")]
pub mod metal_macos;

#[cfg(target_os = "ios")]
pub mod metal_ios;

#[cfg(target_os = "windows")]
pub mod d3d;

#[cfg(target_os = "windows")]
pub mod angle;

mod platform_compositor;

#[cfg(all(feature = "egl", target_os = "android"))]
pub mod egl_android;
#[cfg(feature = "wayland")]
pub mod egl_wayland;
#[cfg(feature = "gl")]
pub mod gl_interface;
#[cfg(feature = "gl")]
pub mod gl_texture_info;
#[cfg(feature = "x11")]
pub mod gl_x11;
