use super::*;
use skia_safe::{Path};

#[no_mangle]
pub fn skia_path_new() -> *mut Path {
    CBox::into_raw(Path::new())
}

#[no_mangle]
pub fn skia_path_drop(_ptr: *mut Path) {
    CBox::drop(_ptr);
}

pub fn skia_move_to(_path: *mut Path) {}