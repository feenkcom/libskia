use array_box::ArrayBox;
use skia_safe::{scalar, PathEffect};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_path_effect_dash(
    intervals_ptr: BorrowedPtr<ArrayBox<f32>>,
    phase: scalar,
) -> OwnedPtr<PathEffect> {
    intervals_ptr
        .with_ref_ok(
            |intervals| match PathEffect::dash(intervals.to_slice(), phase) {
                None => OwnedPtr::null(),
                Some(effect) => OwnedPtr::new(effect),
            },
        )
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_path_effect_corner(radius: scalar) -> OwnedPtr<PathEffect> {
    match PathEffect::corner_path(radius) {
        None => OwnedPtr::null(),
        Some(effect) => OwnedPtr::new(effect),
    }
}

#[no_mangle]
pub fn skia_path_effect_drop(ptr: OwnedPtr<PathEffect>) {
    drop(ptr);
}
