use crate::gpu::angle::SAMPLE_COUNT;

use anyhow::{bail, Result};
use mozangle::egl::ffi::types;
use mozangle::egl::ffi::types::EGLenum;
use mozangle::egl::ffi::*;
use mozangle::egl::get_proc_address;
use std::mem::transmute;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HDC;

pub const PLATFORM_ANGLE_TYPE_D3D11_ANGLE: EGLint = 0x3208;
pub const PLATFORM_ANGLE_ENABLE_AUTOMATIC_TRIM_ANGLE: EGLint = 0x320F;
pub const PLATFORM_ANGLE_DEVICE_TYPE_D3D_WARP_ANGLE: EGLint = 0x320B;
pub const FIXED_SIZE_ANGLE: EGLint = 0x3201;
pub const FRAMEBUFFER_BINDING: EGLenum = 0x8CA6;

const fn int(value: EGLenum) -> EGLint {
    value as EGLint
}

const DISPLAY_CONFIGS: [&'static [EGLint]; 2] = [
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

pub(crate) fn get_display(hdc: HDC) -> Result<(types::EGLDisplay, EGLint, EGLint)> {
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
        bail!("Failed to terminate egl display")
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
