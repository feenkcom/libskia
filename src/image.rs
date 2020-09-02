use boxer::array::BoxerArrayU8;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::gpu::gl::Enum;
use skia_safe::gpu::gl::TextureInfo;
use skia_safe::gpu::{BackendTexture, Context, MipMapped, SurfaceOrigin};
use skia_safe::image::CachingHint;
use skia_safe::prelude::*;
use skia_safe::{
    AlphaType, ColorSpace, ColorType, Data, EncodedImageFormat, FilterQuality, IPoint, ISize,
    Image, ImageInfo, Matrix, Paint, Surface,
};
use std::ffi::c_void;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[no_mangle]
pub fn skia_image_from_pixels(
    pixels_ptr: *mut ValueBox<BoxerArrayU8>,
    width: i32,
    height: i32,
    row_bytes: usize,
    color_type: ColorType,
) -> *mut ValueBox<Image> {
    let image_info = ImageInfo::new(
        ISize::new(width, height),
        color_type,
        AlphaType::Unpremul,
        None,
    );
    pixels_ptr.with_not_null_return(std::ptr::null_mut(), |array| {
        match Image::from_raster_data(&image_info, Data::new_copy(array.to_slice()), row_bytes) {
           None => {
               if cfg!(debug_assertions) {
                   eprintln!("[skia_image_from_pixels] Could not create image from bitmap with width: {:?} height: {:?} bytes per row: {:?} color type: {:?}", width, height, row_bytes, color_type);
               };
               std::ptr::null_mut()
           },
           Some(image) => { ValueBox::new(image).into_raw() }
       }
    })
}

#[no_mangle]
pub fn skia_image_from_file(boxer_string_ptr: *mut ValueBox<BoxerString>) -> *mut ValueBox<Image> {
    boxer_string_ptr.with_not_null_return(std::ptr::null_mut(), |boxer_string| {
        let file_name = boxer_string.to_string();
        let file = File::open(file_name);
        if file.is_err() {
            return std::ptr::null_mut();
        }
        let mut file = file.unwrap();
        let mut buffer = vec![];
        if file.read_to_end(&mut buffer).is_err() {
            return std::ptr::null_mut();
        }

        let data = Data::new_copy(&buffer);
        let my_image = Image::from_encoded(data, None);
        if my_image.is_none() {
            return std::ptr::null_mut();
        }
        let my_image = my_image.unwrap();

        ValueBox::new(my_image).into_raw()
    })
}

#[no_mangle]
pub fn skia_image_from_buffer(
    buffer_ptr: *mut ValueBox<BoxerArrayU8>,
    start: usize,
    end: usize,
) -> *mut ValueBox<Image> {
    buffer_ptr.with_not_null_return(
        std::ptr::null_mut(),
        |buffer| match Image::decode_to_raster(&buffer.to_slice()[start..end], None) {
            None => std::ptr::null_mut(),
            Some(image) => ValueBox::new(image).into_raw(),
        },
    )
}

#[no_mangle]
pub fn skia_image_to_file(
    image_ptr: *mut ValueBox<Image>,
    name_boxer_string_ptr: *mut ValueBox<BoxerString>,
    encoding: EncodedImageFormat,
    quality: i32,
) -> i32 {
    image_ptr.with_not_null_return(-1, |image| {
        name_boxer_string_ptr.with_not_null_return(-1, |name_boxer_string| {
            let file_name = name_boxer_string.to_string();

            let encoded = image.encode_to_data_with_quality(encoding, quality);
            if encoded.is_none() {
                return -2;
            }
            let encoded = encoded.unwrap();

            let file = File::create(file_name);
            if file.is_err() {
                return -3;
            }
            let mut file = file.unwrap();
            if file.write_all(&encoded).is_err() {
                return -4;
            }

            return 0;
        })
    })
}

#[no_mangle]
pub fn skia_scale_image(
    image_ptr: *mut ValueBox<Image>,
    new_x: i32,
    new_y: i32,
    keep_aspect_ratio: bool,
    filter_quality: FilterQuality,
) -> *mut ValueBox<Image> {
    image_ptr.with_not_null_return(std::ptr::null_mut(), |image| {
        let mut resize_x = (new_x as f32) / (image.width() as f32);
        let mut resize_y = (new_y as f32) / (image.height() as f32);
        let mut actual_x = new_x;
        let mut actual_y = new_y;
        if keep_aspect_ratio {
            let resize = resize_x.min(resize_y);
            resize_x = resize;
            resize_y = resize;
            actual_x = (resize_x * (image.width() as f32)) as i32;
            actual_y = (resize_y * (image.height() as f32)) as i32;
        }

        let dimensions = ISize::new(actual_x, actual_y);
        let surface = Surface::new_raster_n32_premul(dimensions);
        if surface.is_none() {
            return std::ptr::null_mut();
        }
        let mut surface = surface.unwrap();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_filter_quality(filter_quality);
        surface
            .canvas()
            .set_matrix(&Matrix::scale((resize_x, resize_y)));
        surface
            .canvas()
            .draw_image(image, IPoint::new(0, 0), Some(&paint));
        surface.canvas().flush();
        let out_image = surface.image_snapshot();

        ValueBox::new(out_image).into_raw()
    })
}

#[no_mangle]
pub fn skia_image_from_texture(
    context_ptr: *mut ValueBox<Context>,
    target: Enum,
    id: Enum,
    width: i32,
    height: i32,
    color_type: ColorType,
    alpha_type: AlphaType,
    release_proc: Option<unsafe extern "C" fn(*mut c_void)>,
    release_context: *mut c_void,
) -> *mut ValueBox<Image> {
    context_ptr.with_not_null_return(std::ptr::null_mut(), |context| {
        let texture_info = TextureInfo::from_target_and_id(target, id);
        let backend_texture =
            unsafe { BackendTexture::new_gl((width, height), MipMapped::No, texture_info) };

        let texture = Image::from_texture_with_release_proc(
            context,
            &backend_texture,
            SurfaceOrigin::BottomLeft,
            color_type,
            alpha_type,
            None,
            release_proc,
            release_context,
        );

        match texture {
            None => std::ptr::null_mut(),
            Some(texture) => ValueBox::new(texture).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_image_from_backend_texture(
    _context_ptr: *mut ValueBox<Context>,
    _backend_texture_ptr: *mut ValueBox<BackendTexture>,
    color_type: ColorType,
    alpha_type: AlphaType,
    release_proc: Option<unsafe extern "C" fn(*mut c_void)>,
    release_context: *mut c_void,
) -> *mut ValueBox<Image> {
    _context_ptr.with_not_null_return(std::ptr::null_mut(), |context| {
        _backend_texture_ptr.with_not_null_return(std::ptr::null_mut(), |backend_texture| {
            let texture = Image::from_texture_with_release_proc(
                context,
                backend_texture,
                SurfaceOrigin::BottomLeft,
                color_type,
                alpha_type,
                None,
                release_proc,
                release_context,
            );

            match texture {
                None => std::ptr::null_mut(),
                Some(texture) => ValueBox::new(texture).into_raw(),
            }
        })
    })
}

#[no_mangle]
pub fn skia_image_get_ref_count(image_ptr: *mut ValueBox<Image>) -> usize {
    image_ptr.with_not_null_return(0, |image| image.as_ref().native()._ref_cnt())
}

#[no_mangle]
pub fn skia_image_get_image_info(image_ptr: *mut ValueBox<Image>) -> *mut ValueBox<ImageInfo> {
    image_ptr.with(
        || ValueBox::new(ImageInfo::default()).into_raw(),
        |image| ValueBox::new(image.image_info().clone()).into_raw(),
    )
}

#[no_mangle]
pub fn skia_image_get_width(image_ptr: *mut ValueBox<Image>) -> i32 {
    image_ptr.with_not_null_return(0, |image| image.width())
}

#[no_mangle]
pub fn skia_image_get_height(image_ptr: *mut ValueBox<Image>) -> i32 {
    image_ptr.with_not_null_return(0, |image| image.height())
}

#[no_mangle]
pub fn skia_image_get_unique_id(image_ptr: *mut ValueBox<Image>) -> u32 {
    image_ptr.with_not_null_return(0, |image| image.unique_id())
}

#[no_mangle]
pub fn skia_image_get_alpha_type(image_ptr: *mut ValueBox<Image>) -> AlphaType {
    image_ptr.with_not_null_return(AlphaType::Unknown, |image| image.alpha_type())
}

#[no_mangle]
pub fn skia_image_get_color_type(image_ptr: *mut ValueBox<Image>) -> ColorType {
    image_ptr.with_not_null_return(ColorType::Unknown, |image| image.color_type())
}

#[no_mangle]
pub fn skia_image_get_color_space(image_ptr: *mut ValueBox<Image>) -> *mut ValueBox<ColorSpace> {
    image_ptr.with(
        || ValueBox::new(ColorSpace::new_srgb()).into_raw(),
        |image| ValueBox::new(image.color_space()).into_raw(),
    )
}

#[no_mangle]
pub fn skia_image_is_alpha_only(image_ptr: *mut ValueBox<Image>) -> bool {
    image_ptr.with_not_null_return(false, |image| image.is_alpha_only())
}

#[no_mangle]
pub fn skia_image_is_opaque(image_ptr: *mut ValueBox<Image>) -> bool {
    image_ptr.with_not_null_return(false, |image| image.is_opaque())
}

#[no_mangle]
pub fn skia_image_is_texture_backend(image_ptr: *mut ValueBox<Image>) -> bool {
    image_ptr.with_not_null_return(false, |image| image.is_texture_backed())
}

#[no_mangle]
pub fn skia_image_get_backend_texture(
    image_ptr: *mut ValueBox<Image>,
) -> *mut ValueBox<BackendTexture> {
    image_ptr.with_not_null_return(std::ptr::null_mut(), |image| {
        let result = image.backend_texture(true);
        ValueBox::new(result.0).into_raw()
    })
}

#[no_mangle]
pub fn skia_image_get_backend_texture_origin(image_ptr: *mut ValueBox<Image>) -> SurfaceOrigin {
    image_ptr.with_not_null_return(SurfaceOrigin::BottomLeft, |image| {
        let result = image.backend_texture(true);
        result.1
    })
}

#[no_mangle]
pub fn skia_image_read_all_pixels(
    surface_ptr: *mut ValueBox<Image>,
    pixels_ptr: *mut ValueBox<BoxerArrayU8>,
) -> bool {
    surface_ptr.with_not_null_return(false, |surface| {
        pixels_ptr.with_not_null_return(false, |pixels| {
            let image_info = surface.image_info();
            let row_bytes = image_info.min_row_bytes();
            surface.read_pixels(
                &image_info,
                pixels.to_slice(),
                row_bytes,
                IPoint::new(0, 0),
                CachingHint::Disallow,
            )
        })
    })
}

#[no_mangle]
pub fn skia_image_drop(mut ptr: *mut ValueBox<Image>) {
    drop!(ptr);
}
