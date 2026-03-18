use array_box::ArrayBox;
use skia_safe::{AlphaType, Canvas, ColorType, IPoint, ISize, Image, ImageInfo, Surface, surfaces};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_new_raster_direct(
    image_info_ptr: BorrowedPtr<ImageInfo>,
    mut pixels_ptr: BorrowedPtr<ArrayBox<u8>>,
    row_bytes: usize,
) -> OwnedPtr<Surface> {
    image_info_ptr
        .with_clone(|image_info| {
            pixels_ptr.with_mut_ok(|pixels| {
                let surface_option = surfaces::wrap_pixels(
                    &image_info,
                    pixels.to_slice_mut(),
                    Some(row_bytes),
                    None,
                );
                match surface_option {
                    None => OwnedPtr::null(),
                    Some(borrows_surface) => {
                        let surface = unsafe { borrows_surface.release() };
                        OwnedPtr::new(surface)
                    }
                }
            })
        })
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_new_raster_n32_premul(width: i32, height: i32) -> OwnedPtr<Surface> {
    let surface_option = surfaces::raster_n32_premul(ISize::new(width, height));
    match surface_option {
        None => OwnedPtr::null(),
        Some(surface) => OwnedPtr::new(surface),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_new_default() -> OwnedPtr<Surface> {
    let surface_option = surfaces::raster_n32_premul(ISize::new(600, 400));
    match surface_option {
        None => OwnedPtr::null(),
        Some(surface) => OwnedPtr::new(surface),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_new_similar(
    mut surface: BorrowedPtr<Surface>,
    ptr_image_info: BorrowedPtr<ImageInfo>,
) -> OwnedPtr<Surface> {
    surface
        .with_mut(|surface| {
            ptr_image_info.with_ref_ok(|image_info| {
                let surface_option = surface.new_surface(image_info);
                match surface_option {
                    None => {
                        if cfg!(debug_assertions) {
                            eprintln!("[skia_surface_new_similar] could not create a surface width: {:?} height: {:?} color type {:?} alpha type {:?}", image_info.width(), image_info.height(), image_info.color_type(), image_info.alpha_type());
                        }
                        OwnedPtr::null()
                    }
                    Some(surface) => OwnedPtr::new(surface),
                }
            })
        })
        .or_log(OwnedPtr::null())
}

/// # Safety
///
/// The returned [`BorrowedPtr<Canvas>`] is borrowed from `surface`. Its
/// lifetime is bound to `Surface`, so it must not outlive the owning `Surface`.
#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_get_canvas(
    mut surface: BorrowedPtr<Surface>,
) -> BorrowedPtr<Canvas> {
    surface
        .with_mut_ok(|surface| BorrowedPtr::from_ref(surface.canvas()))
        .or_log(BorrowedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_get_width(surface: BorrowedPtr<Surface>) -> i32 {
    surface.with_ref_ok(|surface| surface.width()).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_get_color_type(mut surface: BorrowedPtr<Surface>) -> ColorType {
    surface
        .with_mut_ok(|surface| surface.image_info().color_type())
        .or_log(ColorType::Unknown)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_get_alpha_type(mut surface: BorrowedPtr<Surface>) -> AlphaType {
    surface
        .with_mut_ok(|surface| surface.image_info().alpha_type())
        .or_log(AlphaType::Unknown)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_get_height(surface: BorrowedPtr<Surface>) -> i32 {
    surface.with_ref_ok(|surface| surface.height()).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_get_image_info(
    mut surface: BorrowedPtr<Surface>,
) -> OwnedPtr<ImageInfo> {
    surface
        .with_mut_ok(|surface| OwnedPtr::new(surface.image_info()))
        .or_log(OwnedPtr::new(ImageInfo::default()))
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_read_all_pixels(
    mut surface: BorrowedPtr<Surface>,
    mut pixels_ptr: BorrowedPtr<ArrayBox<u8>>,
) -> bool {
    surface
        .with_mut(|surface| {
            pixels_ptr.with_mut_ok(|pixels| {
                let image_info = surface.image_info();
                let row_bytes = image_info.min_row_bytes();
                surface.read_pixels(
                    &image_info,
                    pixels.to_slice_mut(),
                    row_bytes,
                    IPoint::new(0, 0),
                )
            })
        })
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_get_image_snapshot(
    mut surface: BorrowedPtr<Surface>,
) -> OwnedPtr<Image> {
    surface
        .with_mut_ok(|surface| OwnedPtr::new(surface.image_snapshot()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_flush(mut _surface: BorrowedPtr<Surface>) {
    warn!("[skia_surface_flush] surface flush is deprecated. Use DirectContext instead")
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_surface_drop(ptr: OwnedPtr<Surface>) {
    drop(ptr);
}
