pub mod backend_render_target;
pub mod backend_texture;
pub mod context;
pub mod surface_gpu;
pub mod texture_info;

#[cfg(all(feature = "metal", target_os = "macos"))]
pub mod metal_macos;

#[cfg(all(feature = "metal", target_os = "ios"))]
pub mod metal_ios;

#[cfg(feature = "d3d")]
pub mod d3d;

#[cfg(feature = "angle")]
pub mod angle;
#[cfg(feature = "angle")]
pub mod angle_utils;

#[cfg(all(feature = "metal", target_os = "macos"))]
pub use self::metal_macos::*;

#[cfg(all(feature = "metal", target_os = "ios"))]
pub use self::metal_ios::*;

#[cfg(feature = "d3d")]
pub use self::d3d::*;

#[cfg(feature = "angle")]
pub use self::angle::*;

mod platform_compositor;

pub use platform_compositor::*;
