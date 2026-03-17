use skia_safe::{PixelGeometry, SurfaceProps, SurfacePropsFlags};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_surface_props_default() -> OwnedPtr<SurfaceProps> {
    OwnedPtr::new(SurfaceProps::new(
        SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS,
        PixelGeometry::RGBH,
    ))
}

#[no_mangle]
pub fn skia_surface_props_new(flags: u32, pixel_geometry: PixelGeometry) -> OwnedPtr<SurfaceProps> {
    OwnedPtr::new(SurfaceProps::new(
        SurfacePropsFlags::from_bits_truncate(flags),
        pixel_geometry,
    ))
}

#[no_mangle]
pub fn skia_surface_props_get_pixel_geometry(
    surface_props: BorrowedPtr<SurfaceProps>,
) -> PixelGeometry {
    surface_props
        .with_ref_ok(|surface_props| surface_props.pixel_geometry())
        .or_log(PixelGeometry::default())
}

#[no_mangle]
pub fn skia_surface_props_get_flags(surface_props: BorrowedPtr<SurfaceProps>) -> u32 {
    surface_props
        .with_ref_ok(|surface_props| surface_props.flags().bits())
        .or_log(SurfacePropsFlags::default().bits())
}

#[no_mangle]
pub fn skia_surface_props_drop(mut ptr: OwnedPtr<SurfaceProps>) {
    drop(ptr);
}
