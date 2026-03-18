use std::fs::File;
use std::io::Read;
use std::io::Write;

use array_box::ArrayBox;
use skia_safe::gpu::{BackendTexture, SurfaceOrigin};
use skia_safe::image::CachingHint;
use skia_safe::{
    gpu, images, surfaces, AlphaType, ColorSpace, ColorType, Data, EncodedImageFormat, IPoint,
    ISize, Image, ImageInfo, Paint, M44,
};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_image_from_pixels(
    pixels_ptr: BorrowedPtr<ArrayBox<u8>>,
    width: i32,
    height: i32,
    row_bytes: usize,
    color_type: ColorType,
) -> OwnedPtr<Image> {
    let image_info = ImageInfo::new(
        ISize::new(width, height),
        color_type,
        AlphaType::Unpremul,
        None,
    );
    pixels_ptr
        .with_ref_ok(|array| {
            match images::raster_from_data(&image_info, Data::new_copy(array.to_slice()), row_bytes) {
                None => {
                    if cfg!(debug_assertions) {
                        eprintln!("[skia_image_from_pixels] Could not create image from bitmap with width: {:?} height: {:?} bytes per row: {:?} color type: {:?}", width, height, row_bytes, color_type);
                    };
                    OwnedPtr::null()
                }
                Some(image) => OwnedPtr::new(image),
            }
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_image_from_file(boxer_string_ptr: BorrowedPtr<StringBox>) -> OwnedPtr<Image> {
    boxer_string_ptr
        .with_ref_ok(|boxer_string| {
            let file_name = boxer_string.to_string();
            let file = File::open(file_name);
            if file.is_err() {
                return OwnedPtr::null();
            }
            let mut file = file.unwrap();
            let mut buffer = vec![];
            if file.read_to_end(&mut buffer).is_err() {
                return OwnedPtr::null();
            }

            let data = Data::new_copy(&buffer);
            let my_image = Image::from_encoded(data);
            if my_image.is_none() {
                return OwnedPtr::null();
            }
            let my_image = my_image.unwrap();

            OwnedPtr::new(my_image)
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_image_from_buffer(
    buffer_ptr: BorrowedPtr<ArrayBox<u8>>,
    start: usize,
    end: usize,
) -> OwnedPtr<Image> {
    buffer_ptr
        .with_ref_ok(|buffer| {
            let data = Data::new_copy(&buffer.to_slice()[start..end]);
            match Image::from_encoded(data) {
                None => OwnedPtr::null(),
                Some(image) => OwnedPtr::new(image),
            }
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_image_to_file(
    image_ptr: BorrowedPtr<Image>,
    name_boxer_string_ptr: BorrowedPtr<StringBox>,
    encoding: EncodedImageFormat,
    quality: u32,
) -> i32 {
    image_ptr
        .with_ref(|image| {
            name_boxer_string_ptr.with_ref_ok(|name_boxer_string| {
                let file_name = name_boxer_string.to_string();

                let encoded = image.encode(None, encoding, quality);
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

                0
            })
        })
        .or_log(-1)
}

#[no_mangle]
pub fn skia_scale_image(
    image_ptr: BorrowedPtr<Image>,
    new_x: i32,
    new_y: i32,
    keep_aspect_ratio: bool,
) -> OwnedPtr<Image> {
    image_ptr
        .with_ref_ok(|image| {
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
            let surface = surfaces::raster_n32_premul(dimensions);
            if surface.is_none() {
                return OwnedPtr::null();
            }
            let mut surface = surface.unwrap();
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            surface
                .canvas()
                .set_matrix(&M44::scale(resize_x, resize_y, 1.0));
            surface
                .canvas()
                .draw_image(image, IPoint::new(0, 0), Some(&paint));
            let out_image = surface.image_snapshot();

            OwnedPtr::new(out_image)
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_image_get_image_info(image_ptr: BorrowedPtr<Image>) -> OwnedPtr<ImageInfo> {
    image_ptr
        .with_ref_ok(|image| OwnedPtr::new(image.image_info().clone()))
        .unwrap_or_else(|_| OwnedPtr::new(ImageInfo::default()))
}

#[no_mangle]
pub fn skia_image_get_width(image_ptr: BorrowedPtr<Image>) -> i32 {
    image_ptr.with_ref_ok(|image| image.width()).or_log(0)
}

#[no_mangle]
pub fn skia_image_get_height(image_ptr: BorrowedPtr<Image>) -> i32 {
    image_ptr.with_ref_ok(|image| image.height()).or_log(0)
}

#[no_mangle]
pub fn skia_image_get_unique_id(image_ptr: BorrowedPtr<Image>) -> u32 {
    image_ptr.with_ref_ok(|image| image.unique_id()).or_log(0)
}

#[no_mangle]
pub fn skia_image_get_alpha_type(image_ptr: BorrowedPtr<Image>) -> AlphaType {
    image_ptr
        .with_ref_ok(|image| image.alpha_type())
        .or_log(AlphaType::Unknown)
}

#[no_mangle]
pub fn skia_image_get_color_type(image_ptr: BorrowedPtr<Image>) -> ColorType {
    image_ptr
        .with_ref_ok(|image| image.color_type())
        .or_log(ColorType::Unknown)
}

#[no_mangle]
pub fn skia_image_get_color_space(image: BorrowedPtr<Image>) -> OwnedPtr<ColorSpace> {
    image
        .with_ref_ok(|image| image.color_space().map(|space| OwnedPtr::new(space)))
        .or_log(None)
        .unwrap_or_default()
}

#[no_mangle]
pub fn skia_image_is_alpha_only(image: BorrowedPtr<Image>) -> bool {
    image
        .with_ref_ok(|image| image.is_alpha_only())
        .unwrap_or(false)
}

#[no_mangle]
pub fn skia_image_is_opaque(image: BorrowedPtr<Image>) -> bool {
    image
        .with_ref_ok(|image| image.is_opaque())
        .unwrap_or(false)
}

#[no_mangle]
pub fn skia_image_is_texture_backend(image: BorrowedPtr<Image>) -> bool {
    image
        .with_ref_ok(|image| image.is_texture_backed())
        .unwrap_or(false)
}

#[no_mangle]
pub fn skia_image_get_backend_texture(image_ptr: BorrowedPtr<Image>) -> OwnedPtr<BackendTexture> {
    image_ptr
        .with_ref_ok(
            |image| match gpu::images::get_backend_texture_from_image(image, true) {
                None => OwnedPtr::null(),
                Some(result) => OwnedPtr::new(result.0),
            },
        )
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_image_get_backend_texture_origin(image_ptr: BorrowedPtr<Image>) -> SurfaceOrigin {
    image_ptr
        .with_ref_ok(
            |image| match gpu::images::get_backend_texture_from_image(image, true) {
                None => SurfaceOrigin::TopLeft,
                Some(result) => result.1,
            },
        )
        .or_log(SurfaceOrigin::BottomLeft)
}

#[no_mangle]
pub fn skia_image_read_all_pixels(
    surface_ptr: BorrowedPtr<Image>,
    mut pixels_ptr: BorrowedPtr<ArrayBox<u8>>,
) -> bool {
    surface_ptr
        .with_ref(|surface| {
            pixels_ptr.with_mut_ok(|pixels| {
                let image_info = surface.image_info();
                let row_bytes = image_info.min_row_bytes();
                surface.read_pixels(
                    &image_info,
                    pixels.to_slice_mut(),
                    row_bytes,
                    IPoint::new(0, 0),
                    CachingHint::Disallow,
                )
            })
        })
        .or_log(false)
}

#[no_mangle]
pub fn skia_image_drop(ptr: OwnedPtr<Image>) {
    drop(ptr);
}
