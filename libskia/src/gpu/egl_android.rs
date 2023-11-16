use crate::gpu::{PlatformCompositor, PlatformContext};
use khronos_egl as egl;
use skia_safe::gpu::gl::{Enum, FramebufferInfo, Interface, UInt};
use skia_safe::gpu::MipMapped::No;
use skia_safe::gpu::Protected;
use skia_safe::gpu::{BackendRenderTarget, ContextOptions, DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, ISize, Surface};
use std::error::Error;
use std::ffi::{c_int, c_void};
use value_box::{ValueBox, ValueBoxIntoRaw};

type GLenum = i32;
type GLint = i32;
type GLuint = u32;
type GLsizei = u32;

// See https://chromium.googlesource.com/external/skia/gpu/+/refs/heads/master/include/GrGLDefines.h
const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
const GL_RGBA8: GLenum = 0x8058;
const GL_TRUE: c_int = 1;
const GL_FALSE: c_int = 0;

pub type EglInstance = egl::Instance<egl::Dynamic<libloading::Library, egl::EGL1_4>>;
// See https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGet.xhtml
type GlGetIntegerv = unsafe extern "C" fn(pname: GLenum, data: *mut GLint);
// https://registry.khronos.org/OpenGL-Refpages/es2.0/xhtml/glClearStencil.xml
type GlClearStencil = unsafe extern "C" fn(s: GLint);
type GlViewport = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub struct EglContext {
    native_window: *mut c_void,
    egl: EglInstance,
    gl: Gl,
    egl_context: Option<AndroidWindowContext>,
    width: i32,
    height: i32,
}

impl EglContext {
    pub fn new(
        native_window: *mut c_void,
        width: i32,
        height: i32,
    ) -> Result<Self, Box<dyn Error>> {
        if native_window.is_null() {
            Err("Native window is null")?;
        }

        let lib = unsafe { libloading::Library::new("libEGL.so")? };
        let egl = unsafe { egl::DynamicInstance::<egl::EGL1_4>::load_required_from(lib)? };
        let gl = Gl::new(&egl)?;

        let mut context = Self {
            native_window,
            egl,
            gl,
            egl_context: None,
            width,
            height,
        };

        context.initialize_context()?;
        Ok(context)
    }

    pub fn with_surface(
        &mut self,
        callback: impl FnOnce(&mut Surface),
    ) -> Result<(), Box<dyn Error>> {
        self.make_current()?;

        if let Some(surface) = self.get_surface() {
            callback(surface);
            self.flush_and_submit();
        }
        self.swap_buffers()?;
        self.make_not_current()?;

        Ok(())
    }

    pub fn resize_surface(&mut self, size: ISize) -> Result<(), Box<dyn Error>> {
        self.width = size.width;
        self.height = size.height;

        self.destroy_context()?;
        self.initialize_context()?;
        Ok(())
    }

    fn get_surface(&mut self) -> Option<&mut Surface> {
        if let Some(ref mut egl_context) = self.egl_context {
            if egl_context.surface.is_none() {
                match egl_context.try_create_surface(&self.egl, &self.gl, (self.width, self.height))
                {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Failed to initialize surface: {:?}", error);
                    }
                };
            }
            return egl_context.surface.as_mut();
        }
        None
    }

    fn initialize_context(&mut self) -> Result<(), Box<dyn Error>> {
        if self.egl_context.is_some() {
            Err("Context already initialized")?;
        }
        self.egl_context = Some(AndroidWindowContext::try_create(
            self.native_window,
            &self.egl,
        )?);
        Ok(())
    }

    fn destroy_context(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(mut egl_context) = self.egl_context.take() {
            egl_context.destroy_context(&self.egl)?;
        }
        Ok(())
    }

    fn make_current(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.make_current(&self.egl)?;
        }
        Ok(())
    }

    fn make_not_current(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.make_not_current(&self.egl)?;
        }
        Ok(())
    }

    fn flush_and_submit(&mut self) {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.direct_context.flush_and_submit();
        }
    }

    fn swap_buffers(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut egl_context) = self.egl_context {
            egl_context.swap_buffers(&self.egl)?;
        }
        Ok(())
    }
}

impl Drop for EglContext {
    fn drop(&mut self) {
        match self.destroy_context() {
            Ok(_) => {}
            Err(error) => {
                error!("Failed to destroy context: {}", error)
            }
        }
    }
}

#[derive(Debug)]
struct Gl {
    gl_get_integerv: GlGetIntegerv,
    gl_clear_stencil: GlClearStencil,
    gl_viewport: GlViewport,
}

impl Gl {
    pub fn new(egl: &EglInstance) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            gl_get_integerv: egl
                .get_proc_address("glGetIntegerv")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok_or_else(|| "Could not find glGetIntegerv")?,
            gl_clear_stencil: egl
                .get_proc_address("glClearStencil")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok_or_else(|| "Could not find glClearStencil")?,
            gl_viewport: egl
                .get_proc_address("glViewport")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok_or_else(|| "Could not find glViewport")?,
        })
    }
}

struct AndroidWindowContext {
    egl_display: egl::Display,
    egl_context: egl::Context,
    egl_surface: egl::Surface,
    backend_context: Interface,
    direct_context: DirectContext,
    surface: Option<Surface>,
}

impl AndroidWindowContext {
    fn try_create(native_window: *mut c_void, egl: &EglInstance) -> Result<Self, Box<dyn Error>> {
        let display = egl
            .get_display(egl::DEFAULT_DISPLAY)
            .ok_or_else(|| "Failed to get default display")?;

        let (major, minor) = egl.initialize(display)?;
        egl.bind_api(egl::OPENGL_ES_API)?;

        #[rustfmt::skip]
        let attributes = [
            egl::SURFACE_TYPE, egl::PBUFFER_BIT,
            egl::RENDERABLE_TYPE, egl::OPENGL_ES2_BIT,
            egl::RED_SIZE, 8,
            egl::GREEN_SIZE, 8,
            egl::BLUE_SIZE, 8,
            egl::ALPHA_SIZE, 8,
            egl::STENCIL_SIZE, 8,
            egl::SAMPLE_BUFFERS, 0,
            egl::SAMPLES, 0,
            egl::NONE
        ];

        let config = egl
            .choose_first_config(display, &attributes)?
            .ok_or_else(|| "unable to find an appropriate ELG configuration")?;

        #[rustfmt::skip]
        let context_attributes = [
            egl::CONTEXT_CLIENT_VERSION, 2,
            egl::NONE
        ];

        let context = egl
            .create_context(display, config, None, &context_attributes)
            .map_err(|error| {
                format!(
                    "Failed to create context with attributes {:?}: {}",
                    &context_attributes, error
                )
            })?;
        let surface = unsafe {
            let surface_attributes = None;
            egl.create_window_surface(display, config, native_window, surface_attributes)
                .map_err(|error| {
                    format!(
                        "Failed to create window surface with attributes {:?}: {}",
                        &surface_attributes, error
                    )
                })?
        };

        egl.make_current(display, Some(surface), Some(surface), Some(context))?;

        let interface =
            Interface::new_native().ok_or_else(|| "Failed to create native Interface")?;

        let context_options = ContextOptions::default();
        let direct_context = DirectContext::new_gl(interface.clone(), &context_options)
            .ok_or_else(|| "Failed to create direct context")?;

        Ok(Self {
            egl_display: display,
            egl_context: context,
            egl_surface: surface,
            backend_context: interface,
            direct_context,
            surface: None,
        })
    }

    fn try_create_surface(
        &mut self,
        egl: &EglInstance,
        gl: &Gl,
        size: (i32, i32),
    ) -> Result<(), Box<dyn Error>> {
        if let Some(ref mut surface) = self.surface {
            return Ok(());
        }

        let mut buffer: GLint = 0;
        unsafe { (gl.gl_get_integerv)(GL_FRAMEBUFFER_BINDING, &mut buffer) };

        let framebuffer_info = FramebufferInfo {
            fboid: buffer as UInt,
            format: GL_RGBA8 as Enum,
            protected: Protected::No,
        };

        let backend_render_target = BackendRenderTarget::new_gl(size, 0, 8, framebuffer_info);

        let surface = Surface::from_backend_render_target(
            &mut self.direct_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        );

        self.surface = surface;
        self.surface
            .as_ref()
            .ok_or_else(|| "Failed to create skia Surface")?;
        Ok(())
    }

    fn destroy_context(&mut self, egl: &EglInstance) -> Result<(), Box<dyn Error>> {
        egl.make_current(self.egl_display, None, None, None)?;
        egl.destroy_surface(self.egl_display, self.egl_surface)?;
        egl.destroy_context(self.egl_display, self.egl_context)?;
        unsafe {
            self.egl_context = egl::Context::from_ptr(std::ptr::null_mut());
        }
        unsafe {
            self.egl_surface = egl::Surface::from_ptr(std::ptr::null_mut());
        }
        Ok(())
    }

    fn make_current(&self, egl: &EglInstance) -> Result<(), Box<dyn Error>> {
        egl.make_current(
            self.egl_display,
            Some(self.egl_surface),
            Some(self.egl_surface),
            Some(self.egl_context),
        )?;
        Ok(())
    }

    fn make_not_current(&self, egl: &EglInstance) -> Result<(), Box<dyn Error>> {
        egl.make_current(self.egl_display, None, None, None)?;
        Ok(())
    }

    fn swap_buffers(&self, egl: &EglInstance) -> Result<(), Box<dyn Error>> {
        egl.swap_buffers(self.egl_display, self.egl_surface)?;
        Ok(())
    }
}

#[no_mangle]
pub fn skia_android_egl_compositor_new_size(
    native_window: *mut c_void,
    width: i32,
    height: i32,
) -> *mut ValueBox<PlatformCompositor> {
    EglContext::new(native_window, width, height)
        .map(|context| ValueBox::new(PlatformCompositor::new(PlatformContext::Egl(context))))
        .map_err(|error| error.into())
        .into_raw()
}
