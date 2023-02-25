use skia_safe::gpu::gl::{Enum, FramebufferInfo, Interface, UInt};
use skia_safe::gpu::{BackendRenderTarget, ContextOptions, DirectContext, SurfaceOrigin};
use skia_safe::{ColorType, ISize, Surface};
use std::error::Error;
use std::ffi::{c_void, CString};
use std::fmt::{Display, Formatter};
use std::os::raw::{c_int, c_ulong};
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use std::sync::Mutex;
use value_box::{BoxerError, ValueBox, ValueBoxIntoRaw};

use crate::gpu::{PlatformCompositor, PlatformContext};
use x11::glx;
use x11::glx::{GLXContext, GLXFBConfig};
use x11::xlib;
use x11::xlib::XWindowAttributes;

type GLenum = i32;
type GLint = i32;
type GLuint = u32;
type GLsizei = u32;

#[derive(Clone, Debug)]
pub struct GlConfig {
    pub red_bits: u8,
    pub blue_bits: u8,
    pub green_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,
    pub samples: Option<u8>,
    pub srgb: bool,
    pub double_buffer: bool,
    pub vsync: bool,
}

//   visual  x   bf lv rg d st  colorbuffer  sr ax dp st accumbuffer  ms  cav
//   id dep cl sp  sz l  ci b ro  r  g  b  a F gb bf th cl  r  g  b  a ns b eat
// ----------------------------------------------------------------------------
// 0x05e 24 tc  0  24  0 r  y .   8  8  8  0 .  s  4 24  8 16 16 16 16  0 0 None
impl Default for GlConfig {
    fn default() -> Self {
        GlConfig {
            red_bits: 8,
            blue_bits: 8,
            green_bits: 8,
            alpha_bits: 0,
            depth_bits: 24,
            stencil_bits: 8,
            samples: None,
            srgb: true,
            double_buffer: true,
            vsync: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Compatibility,
    Core,
}

#[derive(Debug)]
pub enum GlError {
    InvalidWindowHandle,
    VersionNotSupported,
    CreationFailed(String),
    FunctionNotFound(String),
}

impl Display for GlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GlError {}

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/GLX_ARB_create_context.txt

type GlXCreateContextAttribsARB = unsafe extern "C" fn(
    dpy: *mut xlib::Display,
    fbc: GLXFBConfig,
    share_context: GLXContext,
    direct: xlib::Bool,
    attribs: *const c_int,
) -> GLXContext;

// See https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_swap_control.txt
type GlXSwapIntervalEXT =
    unsafe extern "C" fn(dpy: *mut xlib::Display, drawable: glx::GLXDrawable, interval: i32);

// See https://registry.khronos.org/OpenGL-Refpages/gl4/html/glGet.xhtml
type GlGetIntegerv = unsafe extern "C" fn(pname: GLenum, data: *mut GLint);
// https://registry.khronos.org/OpenGL-Refpages/es2.0/xhtml/glClearStencil.xml
type GlClearStencil = unsafe extern "C" fn(s: GLint);
type GlViewport = unsafe extern "C" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_framebuffer_sRGB.txt
const GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB: i32 = 0x20B2;

// See https://chromium.googlesource.com/external/skia/gpu/+/refs/heads/master/include/GrGLDefines.h
const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
const GL_RGBA8: GLenum = 0x8058;
const GL_TRUE: c_int = 1;
const GL_FALSE: c_int = 0;

lazy_static! {
    static ref GL_CTX_ERROR_OCCURRED: Mutex<bool> = Mutex::new(false);
}
extern "C" fn err_handler(_dpy: *mut xlib::Display, _err: *mut xlib::XErrorEvent) -> i32 {
    *GL_CTX_ERROR_OCCURRED.lock().unwrap() = true;
    0
}

fn get_proc_address(symbol: &str) -> Result<*const c_void, GlError> {
    let c_string = CString::new(symbol).unwrap();
    let addr =
        unsafe { glx::glXGetProcAddress(c_string.as_ptr() as *const u8).unwrap() as *const c_void };
    if addr.is_null() {
        Err(GlError::FunctionNotFound(symbol.to_string()))
    } else {
        Ok(addr)
    }
}

fn get_proc_address_arb(symbol: &str) -> Result<*const c_void, GlError> {
    let c_string = CString::new(symbol).unwrap();
    let addr = unsafe {
        glx::glXGetProcAddressARB(c_string.as_ptr() as *const u8).unwrap() as *const c_void
    };
    if addr.is_null() {
        Err(GlError::FunctionNotFound(symbol.to_string()))
    } else {
        Ok(addr)
    }
}

#[derive(Debug)]
pub struct XlibGlWindowContext {
    display: *mut xlib::Display,
    window: c_ulong,
    gl: Gl,
    config: GlConfig,
    gl_context: Option<GlContext>,
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct Gl {
    glx_create_context_attribs_arb: Option<GlXCreateContextAttribsARB>,
    glx_swap_interval_ext: Option<GlXSwapIntervalEXT>,
    gl_get_integerv: GlGetIntegerv,
    gl_clear_stencil: GlClearStencil,
    gl_viewport: GlViewport,
}

impl Gl {
    pub fn new() -> Result<Self, GlError> {
        Ok(Self {
            glx_create_context_attribs_arb: get_proc_address_arb("glXCreateContextAttribsARB")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok(),
            glx_swap_interval_ext: get_proc_address_arb("glXSwapIntervalEXT")
                .map(|addr| unsafe { std::mem::transmute(addr) })
                .ok(),
            gl_get_integerv: get_proc_address("glGetIntegerv")
                .map(|addr| unsafe { std::mem::transmute(addr) })?,
            gl_clear_stencil: get_proc_address("glClearStencil")
                .map(|addr| unsafe { std::mem::transmute(addr) })?,
            gl_viewport: get_proc_address("glViewport")
                .map(|addr| unsafe { std::mem::transmute(addr) })?,
        })
    }
}

impl XlibGlWindowContext {
    pub fn create(
        display: *mut c_void,
        window: c_ulong,
        width: i32,
        height: i32,
        config: GlConfig,
    ) -> Result<XlibGlWindowContext, GlError> {
        if display.is_null() {
            return Err(GlError::InvalidWindowHandle);
        }

        let display = display as *mut xlib::_XDisplay;
        let gl = Gl::new()?;

        Ok(XlibGlWindowContext {
            display,
            window,
            gl,
            config,
            gl_context: None,
            width,
            height,
        })
    }

    pub fn with_surface(&mut self, callback: impl FnOnce(&mut Surface)) {
        self.make_current();

        if let Some(surface) = self.get_surface() {
            callback(surface);
            surface.flush_and_submit();
        }
        self.swap_buffers();
    }

    pub fn resize_surface(&mut self, size: ISize) -> Result<(), GlError> {
        self.width = size.width;
        self.height = size.height;

        self.destroy_context();
        self.initialize_context()
    }

    fn make_current(&mut self) {
        if let Some(ref mut gl_context) = self.gl_context {
            unsafe { gl_context.make_current() };
        }
    }

    fn make_not_current(&mut self) {
        if let Some(ref mut gl_context) = self.gl_context {
            unsafe { gl_context.make_not_current() };
        }
    }

    fn swap_buffers(&mut self) {
        if let Some(ref mut gl_context) = self.gl_context {
            unsafe { gl_context.swap_buffers() };
        }
    }

    fn get_surface(&mut self) -> Option<&mut Surface> {
        if let Some(ref mut gl_context) = self.gl_context {
            if gl_context.surface.is_none() {
                match gl_context.try_create_surface(
                    &self.gl,
                    &self.config,
                    (self.width, self.height),
                ) {
                    Ok(_) => {}
                    Err(error) => {
                        error!("Failed to initialize surface: {:?}", error);
                    }
                };
            }
            return gl_context.surface.as_mut();
        }
        None
    }

    fn initialize_context(&mut self) -> Result<(), GlError> {
        info!("Trying to initialize context for {:?}", self);

        let visual_id = {
            let mut attributes = XWindowAttributes {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                border_width: 0,
                depth: 0,
                visual: std::ptr::null_mut(),
                root: 0,
                class: 0,
                bit_gravity: 0,
                win_gravity: 0,
                backing_store: 0,
                backing_planes: 0,
                backing_pixel: 0,
                save_under: 0,
                colormap: 0,
                map_installed: 0,
                map_state: 0,
                all_event_masks: 0,
                your_event_mask: 0,
                do_not_propagate_mask: 0,
                override_redirect: 0,
                screen: std::ptr::null_mut(),
            };
            unsafe { xlib::XGetWindowAttributes(self.display, self.window, &mut attributes) };
            let visual = unsafe { &*attributes.visual };
            let visual_id = visual.visualid;
            drop(attributes);
            visual_id
        };

        let screen = unsafe { xlib::XDefaultScreen(self.display) };

        #[rustfmt::skip]
        let fb_attribs = [
            glx::GLX_VISUAL_ID, visual_id.try_into().unwrap(),
            glx::GLX_X_RENDERABLE, 1,
            glx::GLX_DRAWABLE_TYPE, glx::GLX_WINDOW_BIT,
            0,
        ];

        let mut n_configs = 0;
        let fb_config = unsafe {
            glx::glXChooseFBConfig(self.display, screen, fb_attribs.as_ptr(), &mut n_configs)
        };

        let mut gl_context = unsafe {
            GlContext::try_create(
                self.display,
                self.window,
                &self.gl,
                &self.config,
                if n_configs > 0 { Some(fb_config) } else { None },
            )
        }?;
        gl_context.try_create_direct_context()?;

        self.gl_context = Some(gl_context);
        Ok(())
    }

    fn destroy_context(&mut self) {
        if let Some(mut gl_context) = self.gl_context.take() {
            unsafe { gl_context.destroy_context() }
        }
    }
}

#[derive(Debug)]
struct GlContext {
    display: *mut xlib::Display,
    window: c_ulong,
    glx_context: GLXContext,
    backend_context: Interface,
    direct_context: Option<DirectContext>,
    surface: Option<Surface>,
}

impl GlContext {
    pub unsafe fn try_create(
        display: *mut xlib::Display,
        window: c_ulong,
        gl: &Gl,
        config: &GlConfig,
        fb_config: Option<*mut GLXFBConfig>,
    ) -> Result<Self, GlError> {
        let mut current = false;
        let mut interface: Option<Interface> = None;
        let mut glx_context: GLXContext = std::ptr::null_mut();

        if let Some(ref glx_create_context_attribs_arb) = gl.glx_create_context_attribs_arb {
            if let Some(fb_config) = fb_config {
                let prev_callback = xlib::XSetErrorHandler(Some(err_handler));
                for (major, minor) in Self::versions().iter().rev() {
                    for profile in Self::profiles() {
                        if !glx_context.is_null() {
                            break;
                        }
                        *GL_CTX_ERROR_OCCURRED.lock().unwrap() = false;
                        #[rustfmt::skip]
                        let ctx_attribs = [
                            glx::arb::GLX_CONTEXT_MAJOR_VERSION_ARB, *major as i32,
                            glx::arb::GLX_CONTEXT_MINOR_VERSION_ARB, *minor as i32,
                            glx::arb::GLX_CONTEXT_PROFILE_MASK_ARB, *profile as i32,
                            0,
                        ];

                        glx_context = (glx_create_context_attribs_arb)(
                            display,
                            *fb_config,
                            std::ptr::null_mut(),
                            1,
                            ctx_attribs.as_ptr(),
                        );

                        // // Sync to ensure any errors generated are processed.
                        xlib::XSync(display, 0);
                        if *GL_CTX_ERROR_OCCURRED.lock().unwrap() {
                            continue;
                        }

                        if glx_context.is_null() {
                            continue;
                        }

                        if !glx_context.is_null()
                            && *profile == glx::arb::GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB
                        {
                            if glx::glXMakeCurrent(display, window, glx_context) != 0 {
                                current = true;
                                // Look to see if RenderDoc is attached. If so, re-create the context with a core profile.
                                interface = Interface::new_native();
                                if let Some(ref new_interface) = interface {
                                    if new_interface.has_extension("GL_EXT_debug_tool") {
                                        interface = None;
                                        glx::glXMakeCurrent(display, window, std::ptr::null_mut());
                                        glx::glXDestroyContext(display, glx_context);
                                        current = false;
                                        glx_context = std::ptr::null_mut();
                                    }
                                }
                            }
                        }
                    }
                }
                xlib::XSetErrorHandler(prev_callback);
            }
        }

        if glx_context.is_null() {
            return Err(GlError::CreationFailed(
                "Could not instantiate glx context".to_string(),
            ));
        }

        if !current && glx::glXMakeCurrent(display, window, glx_context) == 0 {
            glx::glXDestroyContext(display, glx_context);
            return Err(GlError::CreationFailed(
                "Failed to make glx context current".to_string(),
            ));
        }

        if let Some(ref glx_swap_interval_ext) = gl.glx_swap_interval_ext {
            (glx_swap_interval_ext)(display, window, if config.vsync { 1 } else { 0 });
        }

        let (mut root, mut x, mut y, mut width, mut height, mut border_width, mut depth) =
            (0, 0, 0, 0, 0, 0, 0);
        xlib::XGetGeometry(
            display,
            window,
            &mut root,
            &mut x,
            &mut y,
            &mut width,
            &mut height,
            &mut border_width,
            &mut depth,
        );
        (gl.gl_viewport)(0, 0, width, height);

        if interface.is_none() {
            interface = Interface::new_native();
        }

        match interface {
            None => Err(GlError::CreationFailed(
                "Could not instantiate native Interface".to_string(),
            )),
            Some(interface) => Ok(Self {
                display,
                window,
                glx_context,
                backend_context: interface,
                direct_context: None,
                surface: None,
            }),
        }
    }

    fn try_create_direct_context(&mut self) -> Result<(), GlError> {
        let context_options = ContextOptions::default();
        let direct_context = DirectContext::new_gl(self.backend_context.clone(), &context_options);
        if direct_context.is_none() {
            return Err(GlError::CreationFailed(
                "Failed to create direct context".to_string(),
            ));
        }
        self.direct_context = direct_context;
        Ok(())
    }

    fn try_create_surface(
        &mut self,
        gl: &Gl,
        config: &GlConfig,
        size: (i32, i32),
    ) -> Result<(), GlError> {
        if let Some(ref mut surface) = self.surface {
            return Ok(());
        }

        if let Some(ref mut direct_context) = self.direct_context {
            let mut buffer: GLint = 0;
            unsafe { (gl.gl_get_integerv)(GL_FRAMEBUFFER_BINDING, &mut buffer) };

            let framebuffer_info = FramebufferInfo {
                fboid: buffer as UInt,
                format: GL_RGBA8 as Enum,
            };

            let backend_render_target = BackendRenderTarget::new_gl(
                size,
                config.samples.map(|samples| samples as usize),
                config.stencil_bits as usize,
                framebuffer_info,
            );

            let surface = Surface::from_backend_render_target(
                direct_context,
                &backend_render_target,
                SurfaceOrigin::BottomLeft,
                ColorType::RGBA8888,
                None,
                None,
            );

            self.surface = surface;
        }
        if self.surface.is_some() {
            Ok(())
        } else {
            Err(GlError::CreationFailed(
                "Failed to create skia Surface".to_string(),
            ))
        }
    }

    unsafe fn make_current(&self) {
        if glx::glXMakeCurrent(self.display, self.window, self.glx_context) == 0 {
            error!("Failed to make context current");
        }
    }

    unsafe fn make_not_current(&self) {
        glx::glXMakeCurrent(self.display, 0, std::ptr::null_mut());
    }

    unsafe fn swap_buffers(&self) {
        glx::glXSwapBuffers(self.display, self.window);
    }

    unsafe fn destroy_context(&mut self) {
        drop(self.surface.take());

        if let Some(mut direct_context) = self.direct_context.take() {
            direct_context.abandon();
            drop(direct_context);
        }

        if self.display.is_null() || self.glx_context.is_null() {
            return;
        }

        glx::glXMakeCurrent(self.display, 0, std::ptr::null_mut());
        glx::glXDestroyContext(self.display, self.glx_context);
        self.glx_context = std::ptr::null_mut();
    }

    #[rustfmt::skip]
    pub fn versions() -> &'static [(u8, u8)] {
        return &[
            (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5),
            (2, 0), (2, 1),
            (3, 0), (3, 1), (3, 2), (3, 3),
            (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6)
        ]
    }

    pub fn profiles() -> &'static [c_int] {
        &[
            glx::arb::GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
            glx::arb::GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
        ]
    }
}

impl Drop for GlContext {
    fn drop(&mut self) {
        unsafe { self.destroy_context() };
    }
}

#[no_mangle]
pub fn skia_xlib_gl_compositor_new_size(
    display: *mut c_void,
    window: c_ulong,
    width: u32,
    height: u32,
) -> *mut ValueBox<PlatformCompositor> {
    XlibGlWindowContext::create(
        display,
        window,
        width as i32,
        height as i32,
        GlConfig::default(),
    )
    .and_then(|mut context| context.initialize_context().map(|_| context))
    .map(|context| ValueBox::new(PlatformCompositor::new(PlatformContext::XlibGl(context))))
    .map_err(|error| BoxerError::AnyError(Box::new(error).into()))
    .into_raw()
}
