use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use skia_safe::{Canvas, Picture, Rect};

#[no_mangle]
pub fn skia_picture_drop(_ptr: *mut ValueBox<Picture>) {
    _ptr.drop();
}

#[no_mangle]
pub fn skia_picture_cull_rect(_ptr: *mut ValueBox<Picture>) -> *mut ValueBox<Rect> {
    _ptr.with_not_null_return(std::ptr::null_mut(), |picture| {
        ValueBox::new(picture.cull_rect()).into_raw()
    })
}

#[no_mangle]
pub fn skia_picture_is_empty(_ptr: *mut ValueBox<Picture>) -> bool {
    _ptr.with_not_null_return(true, |picture| picture.cull_rect().is_empty())
}

#[no_mangle]
pub fn skia_picture_unique_id(_ptr: *mut ValueBox<Picture>) -> u32 {
    _ptr.with_not_null_return(0, |picture| picture.unique_id())
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
