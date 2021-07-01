use boxer::array::BoxerArray;
use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::{Canvas, Picture, Rect};

#[no_mangle]
pub fn skia_picture_cull_rect(picture_ptr: *mut ValueBox<Picture>) -> *mut ValueBox<Rect> {
    picture_ptr.with_not_null_return(std::ptr::null_mut(), |picture| {
        ValueBox::new(picture.cull_rect()).into_raw()
    })
}

#[no_mangle]
pub fn skia_picture_is_empty(picture_ptr: *mut ValueBox<Picture>) -> bool {
    picture_ptr.with_not_null_return(true, |picture| picture.cull_rect().is_empty())
}

#[no_mangle]
pub fn skia_picture_unique_id(picture_ptr: *mut ValueBox<Picture>) -> u32 {
    picture_ptr.with_not_null_return(0, |picture| picture.unique_id())
}

#[no_mangle]
pub fn skia_picture_approximate_op_count(_ptr_picture: *mut ValueBox<Picture>) -> usize {
    _ptr_picture.with_not_null_return(0, |picture| picture.approximate_op_count())
}

#[no_mangle]
pub fn skia_picture_playback(
    _ptr_picture: *mut ValueBox<Picture>,
    _ptr_canvas: *mut ReferenceBox<Canvas>,
) {
    _ptr_canvas.with_not_null(|canvas| {
        _ptr_picture.with_not_null(|picture| {
            picture.playback(canvas);
        })
    })
}

#[no_mangle]
pub fn skia_picture_serialize(
    picture_ptr: *mut ValueBox<Picture>,
    data_ptr: *mut ValueBox<BoxerArray<u8>>,
) {
    picture_ptr.with_not_null(|picture| {
        data_ptr.with_not_null(|data| {
            data.set_array(picture.serialize().as_bytes());
        })
    });
}

#[no_mangle]
pub fn skia_picture_drop(ptr: &mut *mut ValueBox<Picture>) {
    drop!(ptr);
}
