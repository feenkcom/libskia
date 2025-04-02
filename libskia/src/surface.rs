use array_box::ArrayBox;
use reference_box::ReferenceBox;
use skia_safe::{surfaces, AlphaType, Canvas, ColorType, IPoint, ISize, Image, ImageInfo, Surface};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_surface_new_raster_direct(
    image_info_ptr: *mut ValueBox<ImageInfo>,
    pixels_ptr: *mut ValueBox<ArrayBox<u8>>,
    row_bytes: usize,
) -> *mut ValueBox<Surface> {
    image_info_ptr.with_not_null_return(std::ptr::null_mut(), |image_info| {
        pixels_ptr.with_not_null_return(std::ptr::null_mut(), |pixels| {
            let surface_option =
                surfaces::wrap_pixels(image_info, pixels.to_slice(), Some(row_bytes), None);
            match surface_option {
                None => std::ptr::null_mut(),
                Some(borrows_surface) => {
                    let surface = unsafe { borrows_surface.release() };
                    ValueBox::new(surface).into_raw()
                }
            }
        })
    })
}

#[no_mangle]
pub fn skia_surface_new_raster_n32_premul(width: i32, height: i32) -> *mut ValueBox<Surface> {
    let surface_option = surfaces::raster_n32_premul(ISize::new(width, height));
    match surface_option {
        None => std::ptr::null_mut(),
        Some(surface) => ValueBox::new(surface).into_raw(),
    }
}

#[no_mangle]
pub fn skia_surface_new_default() -> *mut ValueBox<Surface> {
    let surface_option = surfaces::raster_n32_premul(ISize::new(600, 400));
    match surface_option {
        None => std::ptr::null_mut(),
        Some(surface) => ValueBox::new(surface).into_raw(),
    }
}

#[no_mangle]
pub fn skia_surface_new_similar(
    surface: *mut ValueBox<Surface>,
    ptr_image_info: *mut ValueBox<ImageInfo>,
) -> *mut ValueBox<Surface> {
    surface.with_not_null_return(std::ptr::null_mut(), |surface| {
        ptr_image_info.with_not_null_return(std::ptr::null_mut(), |image_info| {
            let surface_option = surface.new_surface(image_info);
            match surface_option {
                None => {
                    if cfg!(debug_assertions) {
                        eprintln!("[skia_surface_new_similar] could not create a surface width: {:?} height: {:?} color type {:?} alpha type {:?}", image_info.width(), image_info.height(), image_info.color_type(), image_info.alpha_type());
                    }
                    std::ptr::null_mut() },
                Some(surface) => ValueBox::new(surface).into_raw(),
            }
        })
    })
}

#[no_mangle]
pub fn skia_surface_get_canvas(surface: *mut ValueBox<Surface>) -> *mut ReferenceBox<Canvas> {
    surface
        .with_mut_ok(|surface| ReferenceBox::new(surface.canvas()).into_raw())
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn skia_surface_get_width(surface: *mut ValueBox<Surface>) -> i32 {
    surface.with_ref_ok(|surface| surface.width()).or_log(0)
}

#[no_mangle]
pub fn skia_surface_get_color_type(surface: *mut ValueBox<Surface>) -> ColorType {
    surface
        .with_mut_ok(|surface| surface.image_info().color_type())
        .or_log(ColorType::Unknown)
}

#[no_mangle]
pub fn skia_surface_get_alpha_type(surface: *mut ValueBox<Surface>) -> AlphaType {
    surface
        .with_mut_ok(|surface| surface.image_info().alpha_type())
        .or_log(AlphaType::Unknown)
}

#[no_mangle]
pub fn skia_surface_get_height(surface: *mut ValueBox<Surface>) -> i32 {
    surface.with_ref_ok(|surface| surface.height()).or_log(0)
}

#[no_mangle]
pub fn skia_surface_get_image_info(surface: *mut ValueBox<Surface>) -> *mut ValueBox<ImageInfo> {
    surface
        .with_mut_ok(|surface| value_box!(surface.image_info()))
        .or_else(|_| Ok(value_box!(ImageInfo::default())))
        .into_raw()
}

#[no_mangle]
pub fn skia_surface_read_all_pixels(
    surface: *mut ValueBox<Surface>,
    pixels_ptr: *mut ValueBox<ArrayBox<u8>>,
) -> bool {
    surface.with_mut(|surface| {
        pixels_ptr.with_mut_ok(|pixels| {
            let image_info = surface.image_info();
            let row_bytes = image_info.min_row_bytes();
            surface.read_pixels(&image_info, pixels.to_slice(), row_bytes, IPoint::new(0, 0))
        })
    }).or_log(false)
}

#[no_mangle]
pub fn skia_surface_get_image_snapshot(surface: *mut ValueBox<Surface>) -> *mut ValueBox<Image> {
    surface
        .with_mut_ok(|surface| ValueBox::new(surface.image_snapshot()))
        .into_raw()
}

#[no_mangle]
pub fn skia_surface_flush(_surface: *mut ValueBox<Surface>) {
    warn!("[skia_surface_flush] surface flush is deprecated. Use DirectContext instead")
}

#[no_mangle]
pub fn skia_surface_drop(ptr: *mut ValueBox<Surface>) {
    ptr.release();
}
