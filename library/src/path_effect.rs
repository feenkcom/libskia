use boxer::array::BoxerArrayF32;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::{scalar, PathEffect};

#[no_mangle]
pub fn skia_path_effect_dash(
    intervals_ptr: *mut ValueBox<BoxerArrayF32>,
    phase: scalar,
) -> *mut ValueBox<PathEffect> {
    intervals_ptr.with_not_null_return(std::ptr::null_mut(), |intervals| {
        match PathEffect::dash(intervals.to_slice(), phase) {
            None => std::ptr::null_mut(),
            Some(effect) => ValueBox::new(effect).into_raw(),
        }
    })
}

#[no_mangle]
pub fn skia_path_effect_corner(radius: scalar) -> *mut ValueBox<PathEffect> {
    match PathEffect::corner_path(radius) {
        None => std::ptr::null_mut(),
        Some(effect) => ValueBox::new(effect).into_raw(),
    }
}

#[no_mangle]
pub fn skia_path_effect_drop(ptr: &mut *mut ValueBox<PathEffect>) {
    drop!(ptr);
}
