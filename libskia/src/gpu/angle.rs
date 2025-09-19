use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::mem::transmute;

use anyhow::{anyhow, bail, Result};
use mozangle::egl::ffi::*;
use mozangle::egl::get_proc_address;
use skia_safe::gpu::gl::{Format, FramebufferInfo, Interface};
use skia_safe::gpu::{
    BackendRenderTarget, ContextOptions, DirectContext, RecordingContext, SurfaceOrigin,
};
use skia_safe::{gpu, ColorType, ISize, Surface};
use value_box::{ValueBox, ValueBoxIntoRaw};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::GetDC;

use crate::gpu::angle_utils::*;
use crate::gpu::{PlatformCompositor, PlatformContext};

pub const SAMPLE_COUNT: u32 = 1;

#[derive(Debug)]
pub struct AngleContext {
    window: HWND,
    egl_display: types::EGLDisplay,
    egl_context: Option<AngleWindowContext>,
    width: i32,
    height: i32,
}

impl AngleContext {
    pub fn new(window: *mut c_void, width: i32, height: i32) -> Result<Self> {
        let window: HWND = unsafe { transmute(window) };

        let (egl_display, _major_version, _minor_version) = get_display(window)?;

        let mut angle_context = Self {
            window,
            egl_display,
            egl_context: None,
            width,
            height,
        };

        angle_context.initialize_context()?;

        info!("Initialized Angle context {:?}", &angle_context);

        Ok(angle_context)
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) -> Result<()> {
        match self.make_current() {
            Ok(_) => {}
            Err(error) => {
                let _ = self.destroy_context();

                let (egl_display, _major_version, _minor_version) = get_display(self.window)?;
                self.egl_display = egl_display;
                self.initialize_context()?;
                self.make_current()?;
            }
        }

        if let Some(surface) = self.get_surface() {
            trace!(
                "About to draw on a surface of size {}x{}",
                surface.width(),
                surface.height()
            );
            callback(surface);
            self.flush_and_submit();
        }
        self.swap_buffers()?;

        Ok(())
    }

    pub fn resize_surface(&mut self, size: ISize) -> Result<()> {
        debug!(
            "About to resize angle context to {}x{}",
            size.width, size.height
        );
        self.width = size.width;
        self.height = size.height;

        self.destroy_context()?;
        self.initialize_context()?;
        Ok(())
    }

    fn get_surface(&mut self) -> Option<&mut Surface> {
        if let Some(ref mut egl_context) = self.egl_context {
            if egl_context.skia_surface.is_none() {
                match egl_context.try_create_surface(self.width, self.height) {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Failed to initialize surface: {:?}", error);
                    }
                };
            }
            return egl_context.skia_surface.as_mut();
        }
        None
    }

    fn initialize_context(&mut self) -> Result<()> {
        if self.egl_context.is_some() {
            bail!("Context already initialized")
        }
        self.egl_context = Some(AngleWindowContext::try_create(
            self.egl_display,
            self.window,
            self.width,
            self.height,
        )?);
        Ok(())
    }

    fn destroy_context(&mut self) -> Result<()> {
        if let Some(mut egl_context) = self.egl_context.take() {
            egl_context.destroy_context()?;
        }
        Ok(())
    }

    fn make_current(&mut self) -> Result<()> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.make_current()?;
        }
        Ok(())
    }

    fn make_not_current(&mut self) -> Result<()> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.make_not_current()?;
        }
        Ok(())
    }

    fn swap_buffers(&mut self) -> Result<()> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.swap_buffers()?;
        }
        Ok(())
    }

    fn flush_and_submit(&mut self) {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.direct_context.flush_and_submit();
        }
    }
}

impl Drop for AngleContext {
    fn drop(&mut self) {
        self.destroy_context()
            .unwrap_or_else(|error| error!("{:?}", error));
        terminate_display(self.egl_display).unwrap_or_else(|error| error!("{:?}", error));
        self.egl_display = NO_DISPLAY;
    }
}

pub struct AngleWindowContext {
    egl_display: types::EGLDisplay,
    egl_config: types::EGLConfig,
    egl_context: types::EGLContext,
    egl_surface: types::EGLSurface,
    backend_context: Interface,
    direct_context: DirectContext,
    skia_surface: Option<Surface>,
}

impl AngleWindowContext {
    fn try_create(
        egl_display: types::EGLDisplay,
        window: HWND,
        width: i32,
        height: i32,
    ) -> Result<Self> {
        let egl_config = choose_config(egl_display)?;
        let egl_context = create_context(egl_display, egl_config)?;
        let egl_surface = create_window_surface(egl_display, egl_config, window, width, height)?;
        make_current(egl_display, egl_surface, egl_surface, egl_context)?;
        let interface = assemble_interface()?;
        let context_options = ContextOptions::default();
        let direct_context = DirectContext::new_gl(interface.clone(), &context_options)
            .ok_or_else(|| anyhow!("Failed to create direct context"))?;

        Ok(Self {
            egl_display,
            egl_config,
            egl_context,
            egl_surface,
            backend_context: interface,
            direct_context,
            skia_surface: None,
        })
    }

    fn try_create_surface(&mut self, width: i32, height: i32) -> Result<()> {
        debug!(
            "About to create a skia surface of size {}x{}",
            width, height
        );
        let skia_surface = create_skia_surface(&mut self.direct_context, width, height)?;
        self.skia_surface = Some(skia_surface);
        Ok(())
    }

    fn make_current(&self) -> Result<()> {
        make_current(
            self.egl_display,
            self.egl_surface,
            self.egl_surface,
            self.egl_context,
        )
    }

    fn make_not_current(&self) -> Result<()> {
        make_current(self.egl_display, NO_SURFACE, NO_SURFACE, NO_CONTEXT)
    }

    fn swap_buffers(&self) -> Result<()> {
        swap_buffers(self.egl_display, self.egl_surface)
    }

    fn destroy_context(&mut self) -> Result<()> {
        self.make_not_current()?;
        destroy_window_surface(self.egl_display, self.egl_surface)?;
        destroy_egl_context(self.egl_display, self.egl_context)?;
        self.egl_context = NO_CONTEXT;
        self.egl_surface = NO_SURFACE;
        Ok(())
    }
}

impl Debug for AngleWindowContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkiaContext")
            .field("skia_surface", &self.skia_surface)
            .finish()
    }
}

fn assemble_interface() -> Result<Interface> {
    Interface::new_load_with(|name| get_proc_address(name))
        .ok_or_else(|| anyhow!("Failed to create interface"))
}

fn create_direct_context(interface: Interface) -> Result<DirectContext> {
    DirectContext::new_gl(interface.clone(), None)
        .ok_or_else(|| anyhow!("Failed to create direct context"))
}

fn create_skia_surface(
    recording_context: &mut RecordingContext,
    width: i32,
    height: i32,
) -> Result<Surface> {
    let framebuffer = get_framebuffer_binding();

    let framebuffer_info = FramebufferInfo {
        fboid: framebuffer.try_into().unwrap(),
        format: Format::RGBA8.into(),
        protected: skia_safe::gpu::Protected::No,
    };

    let backend_render_target = gpu::backend_render_targets::make_gl(
        (width, height),
        SAMPLE_COUNT as usize,
        0,
        framebuffer_info,
    );

    gpu::surfaces::wrap_backend_render_target(
        recording_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .ok_or_else(|| anyhow!("Failed to create skia surface"))
}

#[no_mangle]
pub fn skia_angle_compositor_new_size(
    window: *mut c_void,
    width: u32,
    height: u32,
) -> *mut ValueBox<PlatformCompositor> {
    AngleContext::new(window, width as i32, height as i32)
        .map(|context| ValueBox::new(PlatformCompositor::new(PlatformContext::Angle(context))))
        .map_err(|error| error.into())
        .into_raw()
}
