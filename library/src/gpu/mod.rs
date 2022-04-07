pub mod backend_render_target;
pub mod backend_texture;
pub mod context;
pub mod surface_gpu;
pub mod texture_info;

#[cfg(feature = "metal")]
pub mod metal;

#[cfg(feature = "d3d")]
pub mod d3d;

#[cfg(feature = "metal")]
pub use metal::*;

#[cfg(feature = "d3d")]
pub use d3d::*;

mod platform_compositor;
pub use platform_compositor::*;
