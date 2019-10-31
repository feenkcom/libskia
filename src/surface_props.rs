use skia_safe::{PixelGeometry, SurfacePropsFlags, SurfaceProps};
use boxer::string::BoxerString;
use boxer::CBox;

#[no_mangle]
pub fn skia_surface_props_pixel_geometry_to_string(_enum: PixelGeometry, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_surface_props_pixel_geometry_to_int(_enum: PixelGeometry) -> i32 {
    unsafe { std::mem::transmute(_enum) }
}

#[no_mangle]
pub fn skia_surface_props_default() -> *mut SurfaceProps {
    CBox::into_raw(SurfaceProps::new(
        SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS,
        PixelGeometry::RGBH,
    ))
}

#[no_mangle]
pub fn skia_surface_props_new(flags: u32, pixel_geometry: PixelGeometry) -> *mut SurfaceProps {
    CBox::into_raw(SurfaceProps::new(
        SurfacePropsFlags::from_bits_truncate(flags),
        pixel_geometry,
    ))
}

#[no_mangle]
pub fn skia_surface_props_get_pixel_geometry(_ptr: *mut SurfaceProps) -> PixelGeometry {
    CBox::with_optional_raw(_ptr, |surface_props_option| match surface_props_option {
        None => { PixelGeometry::default() },
        Some(surface_props) => { surface_props.pixel_geometry() },
    })
}

#[no_mangle]
pub fn skia_surface_props_get_flags(_ptr: *mut SurfaceProps) -> u32 {
    CBox::with_optional_raw(_ptr, |surface_props_option| match surface_props_option {
        None => { SurfacePropsFlags::default().bits() },
        Some(surface_props) => { surface_props.flags().bits() },
    })
}

#[no_mangle]
pub fn skia_surface_props_drop(_ptr: *mut SurfaceProps) {
    CBox::drop(_ptr);
}