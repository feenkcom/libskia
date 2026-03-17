use crate::value_box_compat::*;
use array_box::ArrayBox;
use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::{Canvas, Picture, Rect};
use value_box::{BorrowedPtr, OwnedPtr};

#[no_mangle]
pub fn skia_picture_cull_rect(picture_ptr: BorrowedPtr<Picture>) -> OwnedPtr<Rect> {
    picture_ptr.with_not_null_return(OwnedPtr::null(), |picture| {
        OwnedPtr::new(picture.cull_rect())
    })
}

#[no_mangle]
pub fn skia_picture_is_empty(picture_ptr: BorrowedPtr<Picture>) -> bool {
    picture_ptr.with_not_null_return(true, |picture| picture.cull_rect().is_empty())
}

#[no_mangle]
pub fn skia_picture_unique_id(picture_ptr: BorrowedPtr<Picture>) -> u32 {
    picture_ptr.with_not_null_return(0, |picture| picture.unique_id())
}

#[no_mangle]
pub fn skia_picture_approximate_op_count(mut _ptr_picture: BorrowedPtr<Picture>) -> usize {
    _ptr_picture.with_not_null_return(0, |picture| picture.approximate_op_count())
}

#[no_mangle]
pub fn skia_picture_playback(
    mut _ptr_picture: BorrowedPtr<Picture>,
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
    picture_ptr: BorrowedPtr<Picture>,
    data_ptr: BorrowedPtr<ArrayBox<u8>>,
) {
    picture_ptr.with_not_null(|picture| {
        data_ptr.with_not_null(|data| {
            data.set_array(picture.serialize().as_bytes());
        })
    });
}

#[no_mangle]
pub fn skia_picture_drop(mut ptr: OwnedPtr<Picture>) {
    ptr.release();
}
