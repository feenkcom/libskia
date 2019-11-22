use boxer::array::BoxerArrayU8;
use boxer::boxes::{ReferenceBox, ValueBox, ValueBoxPointer};
use skia_safe::{AlphaType, Canvas, ColorType, IPoint, ISize, Image, ImageInfo, Surface};

#[inline]
pub fn assert_surface(surface_ptr: *mut ValueBox<Surface>) {
    if cfg!(debug_assertions) {
        assert!(
            !surface_ptr.is_null(),
            "ValueBox<Surface> pointer should not be nil"
        );
        let value_box = unsafe { boxer::boxes::from_raw(surface_ptr) };
        let pointer = value_box.boxed();
        boxer::boxes::into_raw(value_box);
        assert!(!pointer.is_null(), "Surface pointer should not be nil");
    }
}

#[no_mangle]
pub fn skia_surface_new_raster_direct(
    _image_info_ptr: *mut ValueBox<ImageInfo>,
    _pixels_ptr: *mut ValueBox<BoxerArrayU8>,
    _row_bytes: usize,
) -> *mut ValueBox<Surface> {
    _image_info_ptr.with(|image_info| {
        _pixels_ptr.with(|pixels| {
            let surface_option =
                Surface::new_raster_direct(image_info, pixels.to_slice(), Some(_row_bytes), None);
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
    _surface_ptr: *mut ValueBox<Surface>,
    _ptr_image_info: *mut ValueBox<ImageInfo>,
) -> *mut ValueBox<Surface> {
    assert_surface(_surface_ptr);
    _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
        _ptr_image_info.with_not_null_return(std::ptr::null_mut(), |image_info| {
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
pub fn skia_surface_get_canvas(_surface_ptr: *mut ValueBox<Surface>) -> *mut ReferenceBox<Canvas> {
    assert_surface(_surface_ptr);
    _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
        ReferenceBox::new(surface.canvas()).into_raw()
    })
}

#[no_mangle]
pub fn skia_surface_get_width(_surface_ptr: *mut ValueBox<Surface>) -> i32 {
    _surface_ptr.with_not_null_return(0, |surface| surface.width())
}

#[no_mangle]
pub fn skia_surface_get_color_type(_surface_ptr: *mut ValueBox<Surface>) -> ColorType {
    _surface_ptr.with(|surface| surface.image_info().color_type())
}

#[no_mangle]
pub fn skia_surface_get_alpha_type(_surface_ptr: *mut ValueBox<Surface>) -> AlphaType {
    _surface_ptr.with(|surface| surface.image_info().alpha_type())
}

#[no_mangle]
pub fn skia_surface_get_height(_surface_ptr: *mut ValueBox<Surface>) -> i32 {
    _surface_ptr.with_not_null_return(0, |surface| surface.height())
}

#[no_mangle]
pub fn skia_surface_get_image_info(
    _surface_ptr: *mut ValueBox<Surface>,
) -> *mut ValueBox<ImageInfo> {
    _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
        ValueBox::new(surface.image_info()).into_raw()
    })
}

#[no_mangle]
pub fn skia_surface_read_all_pixels(
    _surface_ptr: *mut ValueBox<Surface>,
    _pixels_ptr: *mut ValueBox<BoxerArrayU8>,
) -> bool {
    _surface_ptr.with_not_null_return(false, |surface| {
        _pixels_ptr.with(|pixels| {
            let image_info = surface.image_info();
            let row_bytes = image_info.min_row_bytes();
            surface.read_pixels(&image_info, pixels.to_slice(), row_bytes, IPoint::new(0, 0))
        })
    })
}

#[no_mangle]
pub fn skia_surface_get_image_snapshot(
    _surface_ptr: *mut ValueBox<Surface>,
) -> *mut ValueBox<Image> {
    _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
        ValueBox::new(surface.image_snapshot()).into_raw()
    })
}

#[no_mangle]
pub fn skia_surface_flush(_ptr: *mut ValueBox<Surface>) {
    _ptr.with_not_null(|surface| surface.flush());
}

#[no_mangle]
pub fn skia_surface_drop(_ptr: *mut ValueBox<Surface>) {
    _ptr.drop()
}
