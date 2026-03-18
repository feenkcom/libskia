use array_box::ArrayBox;
use skia_safe::{Canvas, Picture, Rect};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_cull_rect(picture: BorrowedPtr<Picture>) -> OwnedPtr<Rect> {
    picture
        .with_clone_ok(|picture| OwnedPtr::new(picture.cull_rect()))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_is_empty(picture: BorrowedPtr<Picture>) -> bool {
    picture
        .with_clone_ok(|picture| picture.cull_rect().is_empty())
        .or_log(true)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_unique_id(picture: BorrowedPtr<Picture>) -> u32 {
    picture
        .with_clone_ok(|picture| picture.unique_id())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_approximate_op_count(mut picture: BorrowedPtr<Picture>) -> usize {
    picture
        .with_mut_ok(|picture| picture.approximate_op_count())
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_playback(
    mut picture: BorrowedPtr<Picture>,
    canvas: BorrowedPtr<Canvas>,
) {
    canvas
        .with_ref_ok(|canvas| {
            picture
                .with_mut_ok(|picture| {
                    picture.playback(canvas);
                })
                .log()
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_serialize(
    mut picture: BorrowedPtr<Picture>,
    mut data: BorrowedPtr<ArrayBox<u8>>,
) {
    picture
        .with_mut_ok(|picture| {
            data.with_mut_ok(|data| {
                data.set_array(picture.serialize().as_bytes());
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_picture_drop(picture: OwnedPtr<Picture>) {
    drop(picture);
}
