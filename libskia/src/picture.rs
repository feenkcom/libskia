use array_box::ArrayBox;
use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::{Canvas, Picture, Rect};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_cull_rect(picture_ptr: BorrowedPtr<Picture>) -> OwnedPtr<Rect> {
    picture_ptr
        .with_clone_ok(|picture| OwnedPtr::new(picture.cull_rect()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_is_empty(picture_ptr: BorrowedPtr<Picture>) -> bool {
    picture_ptr
        .with_clone_ok(|picture| picture.cull_rect().is_empty())
        .or_log(true)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_unique_id(picture_ptr: BorrowedPtr<Picture>) -> u32 {
    picture_ptr
        .with_clone_ok(|picture| picture.unique_id())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_approximate_op_count(mut _ptr_picture: BorrowedPtr<Picture>) -> usize {
    _ptr_picture
        .with_mut_ok(|picture| picture.approximate_op_count())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_playback(
    mut _ptr_picture: BorrowedPtr<Picture>,
    _ptr_canvas: *mut ReferenceBox<Canvas>,
) {
    _ptr_canvas.with_not_null(|canvas| {
        _ptr_picture
            .with_mut_ok(|picture| {
                picture.playback(canvas);
            })
            .log()
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_serialize(
    mut picture_ptr: BorrowedPtr<Picture>,
    mut data_ptr: BorrowedPtr<ArrayBox<u8>>,
) {
    picture_ptr
        .with_mut_ok(|picture| {
            data_ptr.with_mut_ok(|data| {
                data.set_array(picture.serialize().as_bytes());
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_drop(ptr: OwnedPtr<Picture>) {
    drop(ptr);
}
