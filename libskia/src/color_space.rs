use skia_safe::ColorSpace;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_color_space_new_srgb() -> OwnedPtr<ColorSpace> {
    OwnedPtr::new(ColorSpace::new_srgb())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_color_space_is_srgb(color_space: BorrowedPtr<ColorSpace>) -> bool {
    color_space
        .with_ref_ok(|color_space| color_space.is_srgb())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_color_space_drop(color_space: OwnedPtr<ColorSpace>) {
    drop(color_space);
}
