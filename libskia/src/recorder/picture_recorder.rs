use skia_safe::{Canvas, Picture, PictureRecorder, Rect, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_recorder_new() -> OwnedPtr<PictureRecorder> {
    OwnedPtr::new(PictureRecorder::new())
}

/// # Safety
///
/// The returned [`BorrowedPtr<Canvas>`] is borrowed from `picture_recorder`
/// and must not outlive that `PictureRecorder` or be used after recording is
/// finished.
#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_recorder_begin_recording(
    mut picture_recorder: BorrowedPtr<PictureRecorder>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
) -> BorrowedPtr<Canvas> {
    picture_recorder
        .with_mut_ok(|recorder| {
            BorrowedPtr::from_ref(
                recorder.begin_recording(Rect::new(left, top, right, bottom), false),
            )
        })
        .or_log(BorrowedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_recorder_finish_recording(
    mut picture_recorder: BorrowedPtr<PictureRecorder>,
) -> OwnedPtr<Picture> {
    picture_recorder
        .with_mut_ok(
            |recorder| match recorder.finish_recording_as_picture(None) {
                None => OwnedPtr::null(),
                Some(picture) => OwnedPtr::new(picture),
            },
        )
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_recorder_drop(picture_recorder: OwnedPtr<PictureRecorder>) {
    drop(picture_recorder);
}
