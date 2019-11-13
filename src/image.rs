use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{Image, ImageInfo, AlphaType, ColorType, ColorSpace, ImageCachingHint, IPoint, Bitmap, ISize};
use boxer::array::BoxerArrayU8;
use std::os::raw::c_void;

#[no_mangle]
pub fn skia_image_from_pixels(mut _pixels_ptr: *mut ValueBox<BoxerArrayU8>, width: i32, height: i32, row_bytes: usize, color_type: ColorType) -> *mut ValueBox<Image> {
    let mut bitmap = Bitmap::new();
    let image_info = ImageInfo::new(ISize::new(width, height), color_type, AlphaType::Unpremul, None);

    let pixels_installed = _pixels_ptr.with_value_consumed(|array| {
        unsafe { bitmap.install_pixels(image_info.as_ref(), array.data as *mut c_void, row_bytes) }
    });

    if !pixels_installed {
        if cfg!(debug_assertions) {
            eprintln!("[skia_image_from_pixels] Could not install pixels for image with width: {:?} height: {:?} bytes per row: {:?} color type: {:?}", width, height, row_bytes, color_type);
        }
        return std::ptr::null_mut()
    }

    match Image::from_bitmap(bitmap.as_ref()) {
        None => {
            if cfg!(debug_assertions) {
                eprintln!("[skia_image_from_pixels] Could not create image from bitmap with width: {:?} height: {:?} bytes per row: {:?} color type: {:?}", width, height, row_bytes, color_type);
            }
            std::ptr::null_mut() },
        Some(image) => { ValueBox::new(image).into_raw() },
    }
}

#[no_mangle]
pub fn skia_image_get_image_info(_image_ptr: *mut ValueBox<Image>) -> *mut ValueBox<ImageInfo> {
    _image_ptr.with(|image| ValueBox::new(image.image_info().clone()).into_raw())
}

#[no_mangle]
pub fn skia_image_get_width(_image_ptr: *mut ValueBox<Image>) -> i32 {
    _image_ptr.with(|image| image.width())
}

#[no_mangle]
pub fn skia_image_get_height(_image_ptr: *mut ValueBox<Image>) -> i32 {
    _image_ptr.with(|image| image.height())
}

#[no_mangle]
pub fn skia_image_get_unique_id(_image_ptr: *mut ValueBox<Image>) -> u32 {
    _image_ptr.with(|image| image.unique_id())
}

#[no_mangle]
pub fn skia_image_get_alpha_type(_image_ptr: *mut ValueBox<Image>) -> AlphaType {
    _image_ptr.with(|image| image.alpha_type())
}

#[no_mangle]
pub fn skia_image_get_color_type(_image_ptr: *mut ValueBox<Image>) -> ColorType {
    _image_ptr.with(|image| image.color_type())
}

#[no_mangle]
pub fn skia_image_get_color_space(_image_ptr: *mut ValueBox<Image>) -> *mut ValueBox<ColorSpace> {
    _image_ptr.with(|image| ValueBox::new(image.color_space()).into_raw())
}

#[no_mangle]
pub fn skia_image_is_alpha_only(_image_ptr: *mut ValueBox<Image>) -> bool {
    _image_ptr.with(|image| image.is_alpha_only())
}

#[no_mangle]
pub fn skia_image_is_opaque(_image_ptr: *mut ValueBox<Image>) -> bool {
    _image_ptr.with(|image| image.is_opaque())
}

#[no_mangle]
pub fn skia_image_is_texture_backed(_image_ptr: *mut ValueBox<Image>) -> bool {
    _image_ptr.with(|image| image.is_texture_backed())
}

#[no_mangle]
pub fn skia_image_read_all_pixels(_surface_ptr: *mut ValueBox<Image>, _pixels_ptr: *mut ValueBox<BoxerArrayU8>) -> bool {
    _surface_ptr.with_not_null_return(false, |surface| {
        _pixels_ptr.with(|pixels| {
            let image_info = surface.image_info();
            let row_bytes = image_info.min_row_bytes();
            surface.read_pixels(&image_info, pixels.to_slice(), row_bytes, IPoint::new(0,0), ImageCachingHint::Allow)
        })
    })
}

#[no_mangle]
pub fn skia_image_drop(_ptr: *mut ValueBox<Image>) {
    _ptr.drop()
}