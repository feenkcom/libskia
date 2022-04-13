use crate::gpu::angle::SAMPLE_COUNT;
use anyhow::{bail, Result};
use mozangle::egl::ffi::types::EGLenum;
use mozangle::egl::ffi::*;
use mozangle::egl::get_proc_address;
use std::mem::transmute;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HDC;

pub const PLATFORM_ANGLE_TYPE_D3D11_ANGLE: EGLint = 0x3208;
pub const FRAMEBUFFER_BINDING: EGLenum = 0x8CA6;

pub(crate) fn get_display(hdc: HDC) -> Result<types::EGLDisplay> {
    // only D3D11 ANGLE is supported.
    let k_type: EGLint = PLATFORM_ANGLE_TYPE_D3D11_ANGLE;
    let attribs: [EGLint; 3] = [
        PLATFORM_ANGLE_TYPE_ANGLE.try_into().unwrap(),
        k_type,
        NONE.try_into().unwrap(),
    ];
    let display =
        unsafe { GetPlatformDisplayEXT(PLATFORM_ANGLE_ANGLE, transmute(hdc), attribs.as_ptr()) };
    if display == NO_DISPLAY {
        bail!("Failed to get platform display");
    } else {
        Ok(display)
    }
}

pub(crate) fn terminate_display(
    display: types::EGLDisplay,
) -> Result<()> {
    if display == NO_DISPLAY {
        bail!("Display is already terminated")
    }

    let result = unsafe { Terminate(display) };

    if result == TRUE {
        Ok(())
    } else {
        bail!("Failed to terminate egl display")
    }
}

pub(crate) fn initialize_display(display: types::EGLDisplay) -> Result<(EGLint, EGLint)> {
    let mut major_version: EGLint = 0;
    let mut minor_version: EGLint = 0;
    let result: types::EGLBoolean =
        unsafe { Initialize(display, &mut major_version, &mut minor_version) };
    if result == TRUE {
        Ok((major_version, minor_version))
    } else {
        bail!("Failed to initialize egl")
    }
}

pub(crate) fn choose_config(display: types::EGLDisplay) -> Result<types::EGLConfig> {
    let sample_buffers = if SAMPLE_COUNT > 1 { 1 } else { 0 };
    let egl_sample_count = if SAMPLE_COUNT > 1 { SAMPLE_COUNT } else { 0 };

    let config_attributes: [EGLint; 15] = [
        RENDERABLE_TYPE,
        // We currently only support ES3.
        OPENGL_ES3_BIT,
        RED_SIZE,
        8,
        GREEN_SIZE,
        8,
        BLUE_SIZE,
        8,
        ALPHA_SIZE,
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
    // We currently only support ES3.
    let context_attributes: [EGLint; 3] = [
        CONTEXT_CLIENT_VERSION.try_into().unwrap(),
        3,
        NONE.try_into().unwrap(),
    ];
    let egl_context: types::EGLContext = unsafe {
        CreateContext(
            display,
            surface_config,
            std::ptr::null_mut(),
            context_attributes.as_ptr(),
        )
    };
    if egl_context == NO_CONTEXT {
        bail!("Failed to create egl context")
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
        bail!("Failed to destroy egl context")
    }
}

pub(crate) fn create_window_surface(
    display: types::EGLDisplay,
    surface_config: types::EGLConfig,
    window: HWND,
) -> Result<types::EGLSurface> {
    let surface: types::EGLSurface = unsafe {
        CreateWindowSurface(
            display,
            surface_config,
            transmute(window),
            std::ptr::null_mut(),
        )
    };
    if surface == NO_SURFACE {
        bail!("Failed to create egl window surface")
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
        bail!("Failed to destroy surface")
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
        bail!("Failed to make current")
    }
}

pub(crate) fn swap_buffers(display: types::EGLDisplay, surface: types::EGLSurface) -> Result<()> {
    let result = unsafe { SwapBuffers(display, surface) };

    if result == TRUE {
        Ok(())
    } else {
        bail!("Failed to swap buffers")
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
