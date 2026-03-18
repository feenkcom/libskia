use reference_box::ReferenceBox;
use skia_safe::{Canvas, Picture, PictureRecorder, Rect, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub fn skia_picture_recorder_new() -> OwnedPtr<PictureRecorder> {
    OwnedPtr::new(PictureRecorder::new())
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub fn skia_picture_recorder_drop(ptr: OwnedPtr<PictureRecorder>) {
    drop(ptr);
}
