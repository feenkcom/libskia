use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::{PixelGeometry, SurfaceProps, SurfacePropsFlags};

#[no_mangle]
pub fn skia_surface_props_default() -> *mut ValueBox<SurfaceProps> {
    ValueBox::new(SurfaceProps::new(
        SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS,
        PixelGeometry::RGBH,
    ))
    .into_raw()
}

#[no_mangle]
pub fn skia_surface_props_new(
    flags: u32,
    pixel_geometry: PixelGeometry,
) -> *mut ValueBox<SurfaceProps> {
    ValueBox::new(SurfaceProps::new(
        SurfacePropsFlags::from_bits_truncate(flags),
        pixel_geometry,
    ))
    .into_raw()
}

#[no_mangle]
pub fn skia_surface_props_get_pixel_geometry(
    surface_props_ptr: *mut ValueBox<SurfaceProps>,
) -> PixelGeometry {
    surface_props_ptr.with_not_null_return(PixelGeometry::default(), |surface_props| {
        surface_props.pixel_geometry()
    })
}

#[no_mangle]
pub fn skia_surface_props_get_flags(surface_props_ptr: *mut ValueBox<SurfaceProps>) -> u32 {
    surface_props_ptr.with_not_null_return(SurfacePropsFlags::default().bits(), |surface_props| {
        surface_props.flags().bits()
    })
}

#[no_mangle]
pub fn skia_surface_props_drop(ptr: *mut ValueBox<SurfaceProps>) {
    ptr.release();
}
