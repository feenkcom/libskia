use skia_safe::{Surface, ImageInfo, ISize};
use boxer::array::BoxerArrayU8;
use boxer::CBox;

#[no_mangle]
pub fn skia_surface_new_raster_direct(
    _image_info_ptr: *mut ImageInfo,
    _pixels_ptr: *mut BoxerArrayU8,
    _row_bytes: usize) -> *mut Surface {
        CBox::with_two_raw(_image_info_ptr, _pixels_ptr, |image_info, pixels| {
            let surface_option = Surface::new_raster_direct(image_info, pixels.to_slice(), Some(_row_bytes), None);
            match surface_option {
                None => { std::ptr::null_mut()},
                Some(borrows_surface) => {
                    let surface = unsafe { borrows_surface.release() };
                    CBox::into_raw(surface)
                },
            }
        })
}

#[no_mangle]
pub fn skia_surface_new_raster_n32_premul(width: i32, height: i32) -> *mut Surface {
    let surface_option = Surface::new_raster_n32_premul(ISize::new(width, height));
    match surface_option {
        None => { std::ptr::null_mut()},
        Some(surface) => { CBox::into_raw(surface) }
    }
}

#[no_mangle]
pub fn skia_surface_new_default() -> *mut Surface {
    let surface_option = Surface::new_raster_n32_premul(ISize::new(600, 400));
    match surface_option {
        None => { std::ptr::null_mut()},
        Some(surface) => { CBox::into_raw(surface) }
    }
}

#[no_mangle]
pub fn skia_surface_drop(_ptr: *mut Surface) {
    CBox::drop(_ptr);
}

#[no_mangle]
pub fn skia_surface_get_width(_surface_ptr: *mut Surface) -> i32 {
    CBox::with_optional_raw(_surface_ptr, |surface_option| {
        match surface_option {
            None => { 0 },
            Some(surface) => { surface.width() },
        }
    })
}

#[no_mangle]
pub fn skia_surface_get_height(_surface_ptr: *mut Surface) -> i32 {
    CBox::with_optional_raw(_surface_ptr, |surface_option| {
        match surface_option {
            None => { 0 },
            Some(surface) => { surface.height() },
        }
    })
}

#[no_mangle]
pub fn skia_surface_get_image_info(_surface_ptr: *mut Surface) -> *mut ImageInfo {
    CBox::with_optional_raw(_surface_ptr, |surface_option| {
        match surface_option {
            None => { std::ptr::null_mut() },
            Some(surface) => { CBox::into_raw(surface.image_info()) },
        }
    })
}