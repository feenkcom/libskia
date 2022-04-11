use boxer::{ReturnBoxerResult, ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use compositor::{Compositor, Layer};
use compositor_skia::{Cache, SkiaCachelessCompositor, SkiaCompositor};
use fps_counter::FPSCounter;
use skia_safe::{Color, Color4f, Font, ISize, Paint, Point, Surface, Typeface};
use std::ops::Deref;
use std::sync::{Arc, Mutex};

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
    pub fn submit_layer(&mut self, layer: Arc<dyn Layer>) {
        self.latest_frame.lock().unwrap().replace(layer);
    }

    pub fn enable_fps(&mut self) {
        self.render_fps.replace(FPSCounter::default());
    }

    pub fn disable_fps(&mut self) {
        self.render_fps.take();
    }

    pub fn draw(&mut self) {
        let current_layer = self.latest_frame.lock().unwrap().clone();

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
    }

    pub fn draw_cacheless(&mut self) {
        let current_layer = self.latest_frame.lock().unwrap().clone();

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
    }
}

pub enum PlatformContext {
    #[cfg(feature = "metal")]
    Metal(crate::gpu::MetalContext),
    #[cfg(feature = "d3d")]
    D3D(crate::gpu::D3D12Context),
    Unsupported,
}

impl PlatformContext {
    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        match self {
            #[cfg(feature = "metal")]
            PlatformContext::Metal(context) => context.with_surface(callback),
            #[cfg(feature = "d3d")]
            PlatformContext::D3D(context) => context.with_surface(callback),
            PlatformContext::Unsupported => {}
        }
    }

    pub fn resize_surface(&mut self, size: ISize) {
        match self {
            #[cfg(feature = "metal")]
            PlatformContext::Metal(context) => context.resize_surface(size),
            #[cfg(feature = "d3d")]
            PlatformContext::D3D(context) => context.resize(size),
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
        .to_ref()
        .and_then(|mut compositor| {
            layer
                .to_ref()
                .map(|layer| compositor.submit_layer(layer.deref().clone()))
        })
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_draw(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.draw())
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_draw_cacheless(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.draw_cacheless())
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_resize(
    compositor: *mut ValueBox<PlatformCompositor>,
    width: u32,
    height: u32,
) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.resize_surface(ISize::new(width as _, height as _)))
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_enable_fps(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.enable_fps())
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_disable_fps(compositor: *mut ValueBox<PlatformCompositor>) {
    compositor
        .to_ref()
        .map(|mut compositor| compositor.disable_fps())
        .log();
}

#[no_mangle]
pub fn skia_platform_compositor_drop(compositor: &mut *mut ValueBox<PlatformCompositor>) {
    compositor.drop();
}
