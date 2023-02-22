use compositor::{Compositor, Layer};
use compositor_skia::{Cache, SkiaCachelessCompositor, SkiaCompositor};
use fps_counter::FPSCounter;
use skia_safe::{Color, Color4f, Font, ISize, Paint, Point, Surface, Typeface};
use std::error::Error;
use std::sync::{Arc, Mutex};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

lazy_static! {
    static ref FPS_FONT: Font = Font::new(Typeface::default(), 60.0);
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

                SkiaCompositor::new(canvas, &mut self.cache).compose(layer);

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
    #[cfg(feature = "metal")]
    Metal(crate::gpu::MetalContext),
    #[cfg(feature = "d3d")]
    D3D(crate::gpu::D3D12Context),
    #[cfg(feature = "angle")]
    Angle(crate::gpu::AngleContext),
    #[cfg(feature = "x11")]
    XlibGl(crate::gpu::XlibGlWindowContext),
    #[cfg(feature = "egl")]
    Egl(crate::gpu::EglContext),
    Unsupported,
}

impl PlatformContext {
    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        match self {
            #[cfg(feature = "metal")]
            PlatformContext::Metal(context) => context.with_surface(callback),
            #[cfg(feature = "d3d")]
            PlatformContext::D3D(context) => context.with_surface(callback),
            #[cfg(feature = "angle")]
            PlatformContext::Angle(context) => context.with_surface(callback),
            #[cfg(feature = "x11")]
            PlatformContext::XlibGl(context) => context.with_surface(callback),
            #[cfg(feature = "egl")]
            PlatformContext::Egl(context) => {}
            PlatformContext::Unsupported => {}
        }
    }

    pub fn resize_surface(&mut self, size: ISize) {
        match self {
            #[cfg(feature = "metal")]
            PlatformContext::Metal(context) => context.resize_surface(size),
            #[cfg(feature = "d3d")]
            PlatformContext::D3D(context) => context.resize(size),
            #[cfg(feature = "angle")]
            PlatformContext::Angle(context) => context
                .resize(size.width, size.height)
                .unwrap_or_else(|error| error!("{}", error)),
            #[cfg(feature = "x11")]
            PlatformContext::XlibGl(context) => context
                .resize_surface(size)
                .unwrap_or_else(|error| error!("Failed to resize surface: {:?}", error)),
            #[cfg(feature = "egl")]
            PlatformContext::Egl(context) => {}
            PlatformContext::Unsupported => {}
        }
    }
}

#[no_mangle]
pub fn skia_platform_compositor_submit_layer(
    compositor: *mut ValueBox<PlatformCompositor>,
    layer: *mut ValueBox<Arc<dyn Layer>>,
) {
    compositor
        .with_mut(|compositor| {
            layer.with_clone(|layer| compositor.submit_layer(layer).map_err(|error| error.into()))
        })
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_draw(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .with_mut(|compositor| compositor.draw().map_err(|error| error.into()))
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_draw_cacheless(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .with_mut(|compositor| compositor.draw_cacheless().map_err(|error| error.into()))
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_resize(
    compositor: *mut ValueBox<PlatformCompositor>,
    width: u32,
    height: u32,
) {
    compositor
        .with_mut_ok(|compositor| compositor.resize_surface(ISize::new(width as _, height as _)))
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_enable_fps(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .with_mut_ok(|compositor| compositor.enable_fps())
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_disable_fps(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .with_mut_ok(|compositor| compositor.disable_fps())
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_drop(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor.release();
}
