use boxer::boxes::{ReferenceBox, ValueBox, ValueBoxPointer};
use skia_safe::{scalar, Canvas, Picture, PictureRecorder, Rect};

#[no_mangle]
pub fn skia_picture_recorder_new() -> *mut ValueBox<PictureRecorder> {
    ValueBox::new(PictureRecorder::new()).into_raw()
}

#[no_mangle]
pub fn skia_picture_recorder_begin_recording(
    _ptr: *mut ValueBox<PictureRecorder>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) -> *mut ReferenceBox<Canvas> {
    _ptr.with_not_null_return(std::ptr::null_mut(), |recorder| {
        ReferenceBox::new(recorder.begin_recording(Rect::new(left, top, right, bottom), None, None))
            .into_raw()
    })
}

#[no_mangle]
pub fn skia_picture_recorder_finish_recording(
    _ptr: *mut ValueBox<PictureRecorder>,
) -> *mut ValueBox<Picture> {
    _ptr.with_not_null_return(std::ptr::null_mut(), |recorder| {
        match recorder.finish_recording_as_picture(None) {
            None => std::ptr::null_mut(),
            Some(picture) => ValueBox::new(picture).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_picture_recorder_drop(_ptr: *mut ValueBox<PictureRecorder>) {
    _ptr.drop();
}
