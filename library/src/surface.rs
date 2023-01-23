use array_box::ArrayBox;
use reference_box::ReferenceBox;
use skia_safe::{AlphaType, Canvas, ColorType, IPoint, ISize, Image, ImageInfo, Surface};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_surface_new_raster_direct(
    image_info_ptr: *mut ValueBox<ImageInfo>,
    pixels_ptr: *mut ValueBox<ArrayBox<u8>>,
    row_bytes: usize,
) -> *mut ValueBox<Surface> {
    image_info_ptr.with_not_null_return(std::ptr::null_mut(), |image_info| {
        pixels_ptr.with_not_null_return(std::ptr::null_mut(), |pixels| {
            let surface_option =
                Surface::new_raster_direct(image_info, pixels.to_slice(), Some(row_bytes), None);
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
    let surface_option = Surface::new_raster_n32_premul(ISize::new(width, height));
    match surface_option {
        None => std::ptr::null_mut(),
        Some(surface) => ValueBox::new(surface).into_raw(),
    }
}

#[no_mangle]
pub fn skia_surface_new_default() -> *mut ValueBox<Surface> {
    let surface_option = Surface::new_raster_n32_premul(ISize::new(600, 400));
    match surface_option {
        None => std::ptr::null_mut(),
        Some(surface) => ValueBox::new(surface).into_raw(),
    }
}

#[no_mangle]
pub fn skia_surface_new_similar(
    surface_ptr: *mut ValueBox<Surface>,
    ptr_image_info: *mut ValueBox<ImageInfo>,
) -> *mut ValueBox<Surface> {
    surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
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
pub fn skia_surface_get_canvas(surface_ptr: *mut ValueBox<Surface>) -> *mut ReferenceBox<Canvas> {
    surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
        ReferenceBox::new(surface.canvas()).into_raw()
    })
}

#[no_mangle]
pub fn skia_surface_get_width(surface_ptr: *mut ValueBox<Surface>) -> i32 {
    surface_ptr.with_not_null_return(0, |surface| surface.width())
}

#[no_mangle]
pub fn skia_surface_get_color_type(surface_ptr: *mut ValueBox<Surface>) -> ColorType {
    surface_ptr.with_not_null_return(ColorType::Unknown, |surface| {
        surface.image_info().color_type()
    })
}

#[no_mangle]
pub fn skia_surface_get_alpha_type(surface_ptr: *mut ValueBox<Surface>) -> AlphaType {
    surface_ptr.with_not_null_return(AlphaType::Unknown, |surface| {
        surface.image_info().alpha_type()
    })
}

#[no_mangle]
pub fn skia_surface_get_height(surface_ptr: *mut ValueBox<Surface>) -> i32 {
    surface_ptr.with_not_null_return(0, |surface| surface.height())
}

#[no_mangle]
pub fn skia_surface_get_image_info(
    surface_ptr: *mut ValueBox<Surface>,
) -> *mut ValueBox<ImageInfo> {
    surface_ptr
        .with_mut_ok(|surface| surface.image_info())
        .or_else(|_| Ok(ImageInfo::default()))
        .into_raw()
}

#[no_mangle]
pub fn skia_surface_read_all_pixels(
    surface_ptr: *mut ValueBox<Surface>,
    pixels_ptr: *mut ValueBox<ArrayBox<u8>>,
) -> bool {
    surface_ptr.with_not_null_return(false, |surface| {
        pixels_ptr.with_not_null_return(false, |pixels| {
            let image_info = surface.image_info();
            let row_bytes = image_info.min_row_bytes();
            surface.read_pixels(&image_info, pixels.to_slice(), row_bytes, IPoint::new(0, 0))
        })
    })
}

#[no_mangle]
pub fn skia_surface_get_image_snapshot(
    surface_ptr: *mut ValueBox<Surface>,
) -> *mut ValueBox<Image> {
    surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
        ValueBox::new(surface.image_snapshot()).into_raw()
    })
}

#[no_mangle]
pub fn skia_surface_flush(surface_ptr: *mut ValueBox<Surface>) {
    surface_ptr.with_not_null(|surface| surface.flush_and_submit());
}

#[no_mangle]
pub fn skia_surface_drop(ptr: *mut ValueBox<Surface>) {
    ptr.release();
}
