use crate::value_box_compat::*;
use reference_box::ReferenceBox;
use skia_safe::{scalar, Canvas, Picture, PictureRecorder, Rect};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_picture_recorder_new() -> OwnedPtr<PictureRecorder> {
    OwnedPtr::new(PictureRecorder::new()).into_raw()
}

#[no_mangle]
pub fn skia_picture_recorder_begin_recording(
    mut picture_recorder_ptr: BorrowedPtr<PictureRecorder>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) -> *mut ReferenceBox<Canvas> {
    picture_recorder_ptr
        .with_mut_ok(|recorder| {
            ReferenceBox::new(recorder.begin_recording(Rect::new(left, top, right, bottom), false))
                .into_raw()
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn skia_picture_recorder_finish_recording(
    mut picture_recorder_ptr: BorrowedPtr<PictureRecorder>,
) -> OwnedPtr<Picture> {
    picture_recorder_ptr
        .with_mut_ok(
            |recorder| match recorder.finish_recording_as_picture(None) {
                None => OwnedPtr::null(),
                Some(picture) => OwnedPtr::new(picture),
            },
        )
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_picture_recorder_drop(mut ptr: OwnedPtr<PictureRecorder>) {
    ptr.release();
}
