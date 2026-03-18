use array_box::ArrayBox;
use skia_safe::{PathEffect, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_path_effect_dash(
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

#[unsafe(no_mangle)]
pub extern "C" fn skia_path_effect_corner(radius: scalar) -> OwnedPtr<PathEffect> {
    match PathEffect::corner_path(radius) {
        None => OwnedPtr::null(),
        Some(effect) => OwnedPtr::new(effect),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_path_effect_drop(ptr: OwnedPtr<PathEffect>) {
    drop(ptr);
}
