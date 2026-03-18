use std::error::Error;
use std::sync::{Arc, Mutex};

use compositor::{Compositor, Layer};
use compositor_skia::{Cache, SkiaCachelessCompositor, SkiaCompositor};
use compositor_skia_platform::Platform;
use fps_counter::FPSCounter;
use lazy_static::lazy_static;
use skia_safe::{Color, Color4f, Font, FontMgr, FontStyle, ISize, Paint, Point, Surface};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

lazy_static! {
    static ref FPS_FONT: Font = Font::new(
        FontMgr::new()
            .legacy_make_typeface("Arial", FontStyle::normal())
            .unwrap(),
        60.0
    );
    static ref FPS_PAINT: Paint = Paint::new(Color4f::from(Color::BLUE), None);
}

pub struct PlatformCompositor {
    context: PlatformContext,
    latest_frame: Mutex<Option<Arc<dyn Layer>>>,
    cache: Cache,
    render_fps: Option<FPSCounter>,
}

impl PlatformCompositor {
    pub fn new(context: PlatformContext) -> Self {
        Self {
            context,
            latest_frame: Mutex::new(None),
            cache: Cache::new(),
            render_fps: None,
        }
    }

    /// Resize the surface we render on. Must only be called from the main thread
    pub fn resize_surface(&mut self, size: ISize) {
        self.cache = Cache::new();
        self.context.resize_surface(size);
    }

    /// Submit the new layer to be rendered next. Can be called from any thread
    pub fn submit_layer(&mut self, layer: Arc<dyn Layer>) -> Result<(), Box<dyn Error>> {
        self.latest_frame
            .lock()
            .map(|mut frame| {
                frame.replace(layer);
            })
            .map_err(|error| format!("Failed to acquire Mutex lock: {}", error).into())
    }

    pub fn enable_fps(&mut self) {
        self.render_fps.replace(FPSCounter::default());
    }

    pub fn disable_fps(&mut self) {
        self.render_fps.take();
    }

    pub fn draw(&mut self) -> Result<(), Box<dyn Error>> {
        let platform = self.context.platform();

        let current_layer = self
            .latest_frame
            .lock()
            .map_err(|error| -> Box<dyn Error> {
                format!("Failed to acquire Mutex lock: {}", error).into()
            })?
            .clone();

        if let Some(layer) = current_layer {
            self.context.with_surface(|surface| {
                let canvas = surface.canvas();
                canvas.clear(Color::WHITE);

                SkiaCompositor::new(platform, canvas, &mut self.cache).compose(layer);

                self.render_fps.as_mut().map(|counter| {
                    canvas.draw_str(
                        &format!("{}", counter.tick()),
                        Point::new(20.0, 70.0),
                        &FPS_FONT,
                        &FPS_PAINT,
                    );
                });
            })
        }

        Ok(())
    }

    pub fn draw_cacheless(&mut self) -> Result<(), Box<dyn Error>> {
        let current_layer = self
            .latest_frame
            .lock()
            .map_err(|error| -> Box<dyn Error> {
                format!("Failed to acquire Mutex lock: {}", error).into()
            })?
            .clone();

        if let Some(layer) = current_layer {
            self.context.with_surface(|surface| {
                let canvas = surface.canvas();
                canvas.clear(Color::WHITE);

                SkiaCachelessCompositor::new(canvas).compose(layer);

                self.render_fps.as_mut().map(|counter| {
                    canvas.draw_str(
                        &format!("{}", counter.tick()),
                        Point::new(20.0, 70.0),
                        &FPS_FONT,
                        &FPS_PAINT,
                    );
                });
            })
        }

        Ok(())
    }
}

pub enum PlatformContext {
    #[cfg(target_os = "macos")]
    Metal(compositor_skia_platform::MetalContext),
    #[cfg(target_os = "ios")]
    Metal(crate::gpu::MetalContext),
    #[cfg(target_os = "windows")]
    D3D(compositor_skia_platform::D3D12Context),
    #[cfg(target_os = "windows")]
    Angle(compositor_skia_platform::AngleContext),
    #[cfg(feature = "x11")]
    XlibGl(crate::gpu::XlibGlWindowContext),
    #[cfg(feature = "egl")]
    Egl(crate::gpu::EglContext),
    Unsupported,
}

impl PlatformContext {
    pub fn platform(&self) -> Option<Platform> {
        match self {
            #[cfg(target_os = "macos")]
            PlatformContext::Metal(context) => Some(Platform::Metal(context.platform())),
            #[cfg(target_os = "windows")]
            PlatformContext::Angle(context) => {
                context.platform().map(|platform| Platform::Angle(platform))
            }
            _ => None,
        }
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        match self {
            #[cfg(target_os = "macos")]
            PlatformContext::Metal(context) => context.with_surface(callback),
            #[cfg(target_os = "ios")]
            PlatformContext::Metal(context) => context.with_surface(callback),
            #[cfg(target_os = "windows")]
            PlatformContext::D3D(context) => context.with_surface(callback),
            #[cfg(target_os = "windows")]
            PlatformContext::Angle(context) => context
                .with_surface(callback)
                .unwrap_or_else(|error| log::error!("Failed to draw on a surface: {:?}", error)),
            #[cfg(feature = "x11")]
            PlatformContext::XlibGl(context) => context.with_surface(callback),
            #[cfg(feature = "egl")]
            PlatformContext::Egl(context) => context
                .with_surface(callback)
                .unwrap_or_else(|error| log::error!("Failed to draw on a surface: {:?}", error)),
            PlatformContext::Unsupported => {}
        }
    }

    pub fn resize_surface(&mut self, size: ISize) {
        match self {
            #[cfg(target_os = "macos")]
            PlatformContext::Metal(context) => context.resize_surface(size),
            #[cfg(target_os = "ios")]
            PlatformContext::Metal(context) => context.resize_surface(size),
            #[cfg(target_os = "windows")]
            PlatformContext::D3D(context) => context.resize(size),
            #[cfg(target_os = "windows")]
            PlatformContext::Angle(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| log::error!("Failed to resize surface: {:?}", error)),
            #[cfg(feature = "x11")]
            PlatformContext::XlibGl(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| log::error!("Failed to resize surface: {:?}", error)),
            #[cfg(feature = "egl")]
            PlatformContext::Egl(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| log::error!("Failed to resize surface: {:?}", error)),
            PlatformContext::Unsupported => {}
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_platform_compositor_submit_layer(
    mut compositor: BorrowedPtr<PlatformCompositor>,
    layer: BorrowedPtr<Arc<dyn Layer>>,
) {
    compositor
        .with_mut(|compositor| {
            layer.with_clone(|layer| compositor.submit_layer(layer).map_err(|error| error.into()))
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_platform_compositor_draw(mut compositor: BorrowedPtr<PlatformCompositor>) {
    compositor
        .with_mut(|compositor| compositor.draw().map_err(|error| error.into()))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_platform_compositor_draw_cacheless(
    mut compositor: BorrowedPtr<PlatformCompositor>,
) {
    compositor
        .with_mut(|compositor| compositor.draw_cacheless().map_err(|error| error.into()))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_platform_compositor_resize(
    mut compositor: BorrowedPtr<PlatformCompositor>,
    width: u32,
    height: u32,
) {
    compositor
        .with_mut_ok(|compositor| compositor.resize_surface(ISize::new(width as _, height as _)))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_platform_compositor_enable_fps(
    mut compositor: BorrowedPtr<PlatformCompositor>,
) {
    compositor
        .with_mut_ok(|compositor| compositor.enable_fps())
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_platform_compositor_disable_fps(
    mut compositor: BorrowedPtr<PlatformCompositor>,
) {
    compositor
        .with_mut_ok(|compositor| compositor.disable_fps())
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_platform_compositor_drop(compositor: OwnedPtr<PlatformCompositor>) {
    drop(compositor);
}
