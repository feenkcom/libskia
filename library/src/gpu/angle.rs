use crate::gpu::angle_utils::*;

use std::ffi::c_void;
use std::fmt::{Debug, Formatter};
use std::mem::transmute;

use crate::gpu::{PlatformCompositor, PlatformContext};
use anyhow::{anyhow, Result};
use boxer::ValueBox;
use mozangle::egl::ffi::*;
use mozangle::egl::get_proc_address;
use skia_safe::gpu::gl::{Format, FramebufferInfo, Interface};
use skia_safe::gpu::{BackendRenderTarget, DirectContext, RecordingContext, SurfaceOrigin};
use skia_safe::{ColorType, Surface};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::GetDC;

pub const SAMPLE_COUNT: u32 = 1;

#[derive(Debug)]
pub struct AngleContext {
    window: HWND,
    display: types::EGLDisplay,
    egl_config: types::EGLConfig,
    egl_context: types::EGLContext,
    egl_surface: types::EGLSurface,
    interface: Interface,
    major_version: EGLint,
    minor_version: EGLint,
    width: i32,
    height: i32,
    skia_context: Option<SkiaContext>,
}

impl AngleContext {
    pub fn new(window: *mut c_void, width: i32, height: i32) -> Result<Self> {
        let window: HWND = unsafe { transmute(window) };

        let hdc = unsafe { GetDC(window) };

        let (display, major_version, minor_version) = get_display(hdc)?;
        let egl_config = choose_config(display)?;
        let context = create_context(display, egl_config)?;
        let egl_surface = create_window_surface(display, egl_config, window, width, height)?;
        make_current(display, egl_surface, egl_surface, context)?;
        let interface = assemble_interface()?;

        let angle_context = Self {
            window,
            display,
            egl_config,
            egl_context: context,
            egl_surface,
            interface,
            major_version,
            minor_version,
            width,
            height,
            skia_context: None,
        };

        info!("Initialized Angle context {:?}", &angle_context);

        Ok(angle_context)
    }

    pub fn major_version(&self) -> u32 {
        self.major_version as u32
    }

    pub fn minor_version(&self) -> u32 {
        self.minor_version as u32
    }

    pub fn make_current(&self) -> Result<()> {
        make_current(
            self.display,
            self.egl_surface,
            self.egl_surface,
            self.egl_context,
        )
    }

    pub fn swap_buffers(&self) -> Result<()> {
        swap_buffers(self.display, self.egl_surface)
    }

    pub fn surface(&mut self) -> Result<Surface> {
        if let Some(ref skia_context) = self.skia_context {
            Ok(skia_context.skia_surface.clone())
        } else {
            let skia_context = self.create_skia_context()?;
            let surface = skia_context.skia_surface.clone();
            self.skia_context = Some(skia_context);
            Ok(surface)
        }
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        self.make_current().expect("Make current");
        let mut surface = self.surface().expect("Get surface");
        callback(&mut surface);
        surface.flush_and_submit();
        self.swap_buffers()
            .unwrap_or_else(|error| error!("{}", error));
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<()> {
        trace!("About to resize angle context to {}x{}", width, height);
        drop(self.skia_context.take());

        self.width = width;
        self.height = height;

        self.clear_egl_context()?;
        self.destroy_egl_surface()?;
        self.initialize_egl_surface()?;

        Ok(())
    }

    fn create_skia_context(&self) -> Result<SkiaContext> {
        let mut direct_context = create_direct_context(self.interface.clone())?;
        let skia_surface = create_skia_surface(&mut direct_context, self.width, self.height)?;

        Ok(SkiaContext {
            direct_context,
            skia_surface,
        })
    }

    fn initialize_egl_surface(&mut self) -> Result<()> {
        self.egl_surface = create_window_surface(
            self.display,
            self.egl_config,
            self.window,
            self.width,
            self.height,
        )?;
        Ok(())
    }

    fn clear_egl_context(&self) -> Result<()> {
        make_current(self.display, NO_SURFACE, NO_SURFACE, self.egl_context)
    }

    fn destroy_egl_surface(&mut self) -> Result<()> {
        destroy_window_surface(self.display, self.egl_surface)?;
        self.egl_surface = NO_SURFACE;
        Ok(())
    }

    fn destroy_egl_context(&mut self) -> Result<()> {
        destroy_egl_context(self.display, self.egl_context)?;
        self.egl_context = NO_CONTEXT;
        Ok(())
    }

    fn terminate_egl_display(&mut self) -> Result<()> {
        terminate_display(self.display)?;
        self.display = NO_DISPLAY;
        Ok(())
    }
}

impl Drop for AngleContext {
    fn drop(&mut self) {
        drop(self.skia_context.take());
        self.clear_egl_context()
            .unwrap_or_else(|error| debug!("{}", error));
        self.destroy_egl_surface()
            .unwrap_or_else(|error| debug!("{}", error));
        self.destroy_egl_context()
            .unwrap_or_else(|error| debug!("{}", error));
        self.terminate_egl_display()
            .unwrap_or_else(|error| debug!("{}", error));
    }
}

pub struct SkiaContext {
    direct_context: DirectContext,
    skia_surface: Surface,
}

impl Debug for SkiaContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SkiaContext")
            .field("skia_surface", &self.skia_surface)
            .finish()
    }
}

impl Drop for SkiaContext {
    fn drop(&mut self) {
        self.direct_context.abandon();
    }
}

fn assemble_interface() -> Result<Interface> {
    Interface::new_load_with(|name| get_proc_address(name))
        .ok_or_else(|| anyhow!("Failed to create interface"))
}

fn create_direct_context(interface: Interface) -> Result<DirectContext> {
    DirectContext::new_gl(Some(interface.clone()), None)
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
    };

    let backend_render_target =
        BackendRenderTarget::new_gl((width, height), SAMPLE_COUNT as usize, 0, framebuffer_info);

    Surface::from_backend_render_target(
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
    ValueBox::new(PlatformCompositor::new(PlatformContext::Angle(
        AngleContext::new(window, width as i32, height as i32).expect("Create angle context"),
    )))
    .into_raw()
}
