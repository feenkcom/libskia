use crate::gpu::angle::SAMPLE_COUNT;
use std::convert::Infallible;

use anyhow::{bail, Result};
use mozangle::egl::ffi::types;
use mozangle::egl::ffi::types::EGLenum;
use mozangle::egl::ffi::*;
use mozangle::egl::get_proc_address;
use num_enum::FromPrimitive;
use std::convert::TryFrom;
use std::mem::transmute;
use thiserror::Error;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{GetDC, HDC};

pub const PLATFORM_ANGLE_TYPE_D3D11_ANGLE: EGLint = 0x3208;
pub const PLATFORM_ANGLE_ENABLE_AUTOMATIC_TRIM_ANGLE: EGLint = 0x320F;
pub const PLATFORM_ANGLE_DEVICE_TYPE_D3D_WARP_ANGLE: EGLint = 0x320B;
pub const FIXED_SIZE_ANGLE: EGLint = 0x3201;
pub const FRAMEBUFFER_BINDING: EGLenum = 0x8CA6;

const fn int(value: EGLenum) -> EGLint {
    value as EGLint
}

const DISPLAY_CONFIGS: [&'static [EGLint]; 2] =
    [
        &[
            int(PLATFORM_ANGLE_TYPE_ANGLE),
            PLATFORM_ANGLE_TYPE_D3D11_ANGLE,
            // EGL_PLATFORM_ANGLE_ENABLE_AUTOMATIC_TRIM_ANGLE is an option that will
            // enable ANGLE to automatically call the IDXGIDevice3::Trim method on
            // behalf of the application when it gets suspended.
            PLATFORM_ANGLE_ENABLE_AUTOMATIC_TRIM_ANGLE,
            int(TRUE),
            // This extension allows angle to render directly on a D3D swapchain
            // in the correct orientation on D3D11.
            int(EXPERIMENTAL_PRESENT_PATH_ANGLE),
            int(EXPERIMENTAL_PRESENT_PATH_FAST_ANGLE),
            int(NONE),
        ],
        // if D3D11 doesn't work we fallback to a Warp Device (https://docs.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-devices-create-warp)
        &[
            int(PLATFORM_ANGLE_TYPE_ANGLE),
            PLATFORM_ANGLE_TYPE_D3D11_ANGLE,
            int(PLATFORM_ANGLE_DEVICE_TYPE_ANGLE),
            PLATFORM_ANGLE_DEVICE_TYPE_D3D_WARP_ANGLE,
            PLATFORM_ANGLE_ENABLE_AUTOMATIC_TRIM_ANGLE,
            int(TRUE),
            int(NONE),
        ],
    ];

#[derive(Error, Debug)]
pub enum AngleError {
    #[error("Failed to terminate a display {0:?}")]
    FailedToTerminateDisplay(types::EGLDisplay, #[source] EGLError),
    #[error("Failed to create an EGL context for display {0:?}")]
    FailedToCreateContext(types::EGLDisplay, #[source] EGLError),
    #[error("Failed to destroy context {1:?} of display {0:?}")]
    FailedToDestroyContext(types::EGLDisplay, types::EGLContext, #[source] EGLError),
    #[error("Failed to create an EGL surface for display {0:?}")]
    FailedToCreateSurface(types::EGLDisplay, #[source] EGLError),
    #[error("Failed to destroy surface {1:?} of display {0:?}")]
    FailedToDestroySurface(types::EGLDisplay, types::EGLSurface, #[source] EGLError),
    #[error("Failed to swap buffers of surface {1:?} and display {0:?}")]
    FailedToSwapBuffers(types::EGLDisplay, types::EGLSurface, #[source] EGLError),
    #[error("Failed to make context {3:?} current with draw surface {1:?} and read surface {2:?} of display {0:?}")]
    FailedToMakeCurrent(
        types::EGLDisplay,
        types::EGLSurface,
        types::EGLSurface,
        types::EGLContext,
        #[source] EGLError,
    ),
}

unsafe impl Send for AngleError {}
unsafe impl Sync for AngleError {}

/// See https://registry.khronos.org/EGL/sdk/docs/man/html/eglGetError.xhtml
#[allow(non_camel_case_types)]
#[derive(Error, Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u32)]
pub enum EGLError {
    #[error("Expected an error, but it was a success")]
    SUCCESS = SUCCESS,
    #[error("EGL is not initialized, or could not be initialized, for the specified EGL display connection.")]
    NOT_INITIALIZED = NOT_INITIALIZED,
    #[error("EGL cannot access a requested resource (for example a context is bound in another thread).")]
    BAD_ACCESS = BAD_ACCESS,
    #[error("EGL failed to allocate resources for the requested operation.")]
    BAD_ALLOC = BAD_ALLOC,
    #[error("An unrecognized attribute or attribute value was passed in the attribute list.")]
    BAD_ATTRIBUTE = BAD_ATTRIBUTE,
    #[error("An EGLContext argument does not name a valid EGL rendering context.")]
    BAD_CONTEXT = BAD_CONTEXT,
    #[error("An EGLConfig argument does not name a valid EGL frame buffer configuration.")]
    BAD_CONFIG = BAD_CONFIG,
    #[error("The current surface of the calling thread is a window, pixel buffer or pixmap that is no longer valid.")]
    BAD_CURRENT_SURFACE = BAD_CURRENT_SURFACE,
    #[error("An EGLDisplay argument does not name a valid EGL display connection.")]
    BAD_DISPLAY = BAD_DISPLAY,
    #[error("An EGLSurface argument does not name a valid surface (window, pixel buffer or pixmap) configured for GL rendering.")]
    BAD_SURFACE = BAD_SURFACE,
    #[error("Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).")]
    BAD_MATCH = BAD_MATCH,
    #[error("One or more argument values are invalid.")]
    BAD_PARAMETER = BAD_PARAMETER,
    #[error("A NativePixmapType argument does not refer to a valid native pixmap.")]
    BAD_NATIVE_PIXMAP = BAD_NATIVE_PIXMAP,
    #[error("A NativeWindowType argument does not refer to a valid native window.")]
    BAD_NATIVE_WINDOW = BAD_NATIVE_WINDOW,
    #[error("A power management event has occurred. The application must destroy all contexts and reinitialise OpenGL ES state and objects to continue rendering.")]
    CONTEXT_LOST = CONTEXT_LOST,
    #[error("Unknown EGL Error: {0}")]
    #[num_enum(catch_all)]
    UNKNOWN(u32),
}

pub(crate) fn get_error() -> EGLError {
    let result = unsafe { GetError() };
    EGLError::from(result as u32)
}

pub(crate) fn get_display(window: HWND) -> Result<(types::EGLDisplay, EGLint, EGLint)> {
    let hdc = unsafe { GetDC(window) };

    for config in &DISPLAY_CONFIGS {
        let display =
            unsafe { GetPlatformDisplayEXT(PLATFORM_ANGLE_ANGLE, transmute(hdc), config.as_ptr()) };
        if display != NO_DISPLAY {
            let mut major_version: EGLint = 0;
            let mut minor_version: EGLint = 0;
            let result: types::EGLBoolean =
                unsafe { Initialize(display, &mut major_version, &mut minor_version) };
            if result == TRUE {
                return Ok((display, major_version, minor_version));
            } else {
                warn!(
                    "Failed to initialize egl for the display with Angle for the config: {:?}",
                    config
                );
            }
        } else {
            warn!(
                "Could not get platform display with Angle for the config: {:?}",
                config
            );
        }
    }

    bail!("Failed to get platform display");
}

pub(crate) fn terminate_display(display: types::EGLDisplay) -> Result<()> {
    if display == NO_DISPLAY {
        bail!("Display is already terminated")
    }

    let result = unsafe { Terminate(display) };

    if result == TRUE {
        Ok(())
    } else {
        Err(AngleError::FailedToTerminateDisplay(display, get_error()).into())
    }
}

pub(crate) fn choose_config(display: types::EGLDisplay) -> Result<types::EGLConfig> {
    let sample_buffers = if SAMPLE_COUNT > 1 { 1 } else { 0 };
    let egl_sample_count = if SAMPLE_COUNT > 1 { SAMPLE_COUNT } else { 0 };

    let config_attributes: [EGLint; 17] = [
        RED_SIZE,
        8,
        GREEN_SIZE,
        8,
        BLUE_SIZE,
        8,
        ALPHA_SIZE,
        8,
        DEPTH_SIZE,
        8,
        STENCIL_SIZE,
        8,
        SAMPLE_BUFFERS,
        sample_buffers,
        SAMPLES,
        egl_sample_count,
        NONE,
    ]
    .into_iter()
    .map(|each| each.try_into().unwrap())
    .collect::<Vec<EGLint>>()
    .try_into()
    .unwrap();

    let mut surface_config: types::EGLConfig = std::ptr::null_mut();
    let mut num_configs: EGLint = 0;
    let result: types::EGLBoolean = unsafe {
        ChooseConfig(
            display,
            config_attributes.as_ptr(),
            &mut surface_config,
            1,
            &mut num_configs,
        )
    };

    if result == TRUE {
        Ok(surface_config)
    } else {
        bail!("Failed to choose egl config")
    }
}

pub(crate) fn create_context(
    display: types::EGLDisplay,
    surface_config: types::EGLConfig,
) -> Result<types::EGLContext> {
    let context_attributes: [EGLint; 3] = [
        CONTEXT_CLIENT_VERSION.try_into().unwrap(),
        2,
        NONE.try_into().unwrap(),
    ];
    let egl_context: types::EGLContext =
        unsafe {
            CreateContext(
                display,
                surface_config,
                std::ptr::null_mut(),
                context_attributes.as_ptr(),
            )
        };
    if egl_context == NO_CONTEXT {
        Err(AngleError::FailedToCreateContext(display, get_error()).into())
    } else {
        Ok(egl_context)
    }
}

pub(crate) fn destroy_egl_context(
    display: types::EGLDisplay,
    context: types::EGLContext,
) -> Result<()> {
    if context == NO_CONTEXT {
        bail!("Context is already destroyed")
    }

    let result = unsafe { DestroyContext(display, context) };

    if result == TRUE {
        Ok(())
    } else {
        Err(AngleError::FailedToDestroyContext(display, context, get_error()).into())
    }
}

pub(crate) fn create_window_surface(
    display: types::EGLDisplay,
    surface_config: types::EGLConfig,
    window: HWND,
    width: i32,
    height: i32,
) -> Result<types::EGLSurface> {
    let surface_attributes: &[EGLint] = &[
        FIXED_SIZE_ANGLE,
        int(TRUE),
        int(WIDTH),
        width,
        int(HEIGHT),
        height,
        int(NONE),
    ];

    let surface: types::EGLSurface = unsafe {
        CreateWindowSurface(
            display,
            surface_config,
            transmute(window),
            surface_attributes.as_ptr(),
        )
    };
    if surface == NO_SURFACE {
        Err(AngleError::FailedToCreateSurface(display, get_error()).into())
    } else {
        Ok(surface)
    }
}

pub(crate) fn destroy_window_surface(
    display: types::EGLDisplay,
    surface: types::EGLSurface,
) -> Result<()> {
    if surface == NO_SURFACE {
        bail!("Surface already destroyed")
    }
    let result = unsafe { DestroySurface(display, surface) };

    if result == TRUE {
        Ok(())
    } else {
        Err(AngleError::FailedToDestroySurface(display, surface, get_error()).into())
    }
}

pub(crate) fn make_current(
    display: types::EGLDisplay,
    draw_surface: types::EGLSurface,
    read_surface: types::EGLSurface,
    context: types::EGLContext,
) -> Result<()> {
    let result = unsafe { MakeCurrent(display, draw_surface, read_surface, context) };

    if result == TRUE {
        Ok(())
    } else {
        Err(AngleError::FailedToMakeCurrent(
            display,
            draw_surface,
            read_surface,
            context,
            get_error(),
        )
        .into())
    }
}

pub(crate) fn swap_buffers(display: types::EGLDisplay, surface: types::EGLSurface) -> Result<()> {
    let result = unsafe { SwapBuffers(display, surface) };

    if result == TRUE {
        Ok(())
    } else {
        Err(AngleError::FailedToSwapBuffers(display, surface, get_error()).into())
    }
}

#[allow(non_snake_case)]
pub(crate) unsafe fn glGetIntegerv(pname: EGLenum, params: *mut EGLint) {
    let get_integer_v: unsafe extern "C" fn(EGLenum, *mut EGLint) =
        transmute(get_proc_address("glGetIntegerv"));

    get_integer_v(pname, params)
}

pub(crate) fn get_framebuffer_binding() -> EGLint {
    let mut buffer: EGLint = 0;
    unsafe { glGetIntegerv(FRAMEBUFFER_BINDING, &mut buffer) };
    buffer
}
