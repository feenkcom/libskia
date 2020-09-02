use boxer::boxes::ReferenceBox;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::{scalar, Canvas, Picture, PictureRecorder, Rect};

#[no_mangle]
pub fn skia_picture_recorder_new() -> *mut ValueBox<PictureRecorder> {
    ValueBox::new(PictureRecorder::new()).into_raw()
}

#[no_mangle]
pub fn skia_picture_recorder_begin_recording(
    picture_recorder_ptr: *mut ValueBox<PictureRecorder>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) -> *mut ReferenceBox<Canvas> {
    picture_recorder_ptr.with_not_null_return(std::ptr::null_mut(), |recorder| {
        ReferenceBox::new(recorder.begin_recording(Rect::new(left, top, right, bottom), None, None))
            .into_raw()
    })
}

#[no_mangle]
pub fn skia_picture_recorder_finish_recording(
    picture_recorder_ptr: *mut ValueBox<PictureRecorder>,
) -> *mut ValueBox<Picture> {
    picture_recorder_ptr.with_not_null_return(std::ptr::null_mut(), |recorder| {
        match recorder.finish_recording_as_picture(None) {
            None => std::ptr::null_mut(),
            Some(picture) => ValueBox::new(picture).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_picture_recorder_drop(ptr: &mut *mut ValueBox<PictureRecorder>) {
    drop!(ptr);
}
