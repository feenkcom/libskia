use skia_safe::{Surface, ImageInfo, ISize, IPoint, Canvas, Image};
use boxer::array::BoxerArrayU8;
use boxer::boxes::{ReferenceBox, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_surface_new_raster_direct(
    _image_info_ptr: *mut ValueBox<ImageInfo>,
    _pixels_ptr: *mut ValueBox<BoxerArrayU8>,
    _row_bytes: usize) -> *mut ValueBox<Surface> {
        _image_info_ptr.with(|image_info|
            _pixels_ptr.with(|pixels| {
                let surface_option = Surface::new_raster_direct(image_info, pixels.to_slice(), Some(_row_bytes), None);
                match surface_option {
                    None => { std::ptr::null_mut()},
                    Some(borrows_surface) => {
                        let surface = unsafe { borrows_surface.release() };
                        ValueBox::new(surface).into_raw()
                    },
                }
            }))
}

#[no_mangle]
pub fn skia_surface_new_raster_n32_premul(width: i32, height: i32) -> *mut ValueBox<Surface> {
    let surface_option = Surface::new_raster_n32_premul(ISize::new(width, height));
    match surface_option {
        None => { std::ptr::null_mut()},
        Some(surface) => { ValueBox::new(surface).into_raw() }
    }
}

#[no_mangle]
pub fn skia_surface_new_default() -> *mut ValueBox<Surface> {
    let surface_option = Surface::new_raster_n32_premul(ISize::new(600, 400));
    match surface_option {
        None => { std::ptr::null_mut()},
        Some(surface) => { ValueBox::new(surface).into_raw() }
    }
}

#[no_mangle]
pub fn skia_surface_new_similar(_surface_ptr: *mut ValueBox<Surface>, width: i32, height: i32) -> *mut ValueBox<Surface> {
     _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| {
         let surface_option = surface.new_surface_with_dimensions(ISize::new(width, height));
         match surface_option {
            None => { std::ptr::null_mut()},
            Some(surface) => { ValueBox::new(surface).into_raw() }
        }
     })
}

#[no_mangle]
pub fn skia_surface_get_canvas(_surface_ptr: *mut ValueBox<Surface>) -> *mut ReferenceBox<Canvas> {
    _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface | ReferenceBox::new(surface.canvas()).into_raw())
}

#[no_mangle]
pub fn skia_surface_get_width(_surface_ptr: *mut ValueBox<Surface>) -> i32 {
    _surface_ptr.with_not_null_return(0, |surface | surface.width())
}

#[no_mangle]
pub fn skia_surface_get_height(_surface_ptr: *mut ValueBox<Surface>) -> i32 {
    _surface_ptr.with_not_null_return(0, |surface | surface.height())
}

#[no_mangle]
pub fn skia_surface_get_image_info(_surface_ptr: *mut ValueBox<Surface>) -> *mut ValueBox<ImageInfo> {
    _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface | ValueBox::new(surface.image_info()).into_raw())
}

#[no_mangle]
pub fn skia_surface_read_all_pixels(_surface_ptr: *mut ValueBox<Surface>, _pixels_ptr: *mut ValueBox<BoxerArrayU8>) -> bool {
    _surface_ptr.with_not_null_return(false, |surface| {
        _pixels_ptr.with(|pixels| {
            let image_info = surface.image_info();
            let row_bytes = image_info.min_row_bytes();
            surface.read_pixels(&image_info, pixels.to_slice(), row_bytes, IPoint::new(0,0))
        })
    })
}

#[no_mangle]
pub fn skia_surface_get_image_snapshot(_surface_ptr: *mut ValueBox<Surface>) -> *mut ValueBox<Image> {
    _surface_ptr.with_not_null_return(std::ptr::null_mut(), |surface| ValueBox::new(surface.image_snapshot()).into_raw())
}

#[no_mangle]
pub fn skia_surface_flush(_ptr: *mut ValueBox<Surface>) {
    _ptr.with_not_null(|surface| surface.flush());
}

#[no_mangle]
pub fn skia_surface_drop(_ptr: *mut ValueBox<Surface>) {
    _ptr.drop()
}
