use skia_safe::ColorSpace;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub fn skia_color_space_new_srgb() -> OwnedPtr<ColorSpace> {
    OwnedPtr::new(ColorSpace::new_srgb())
}

#[unsafe(no_mangle)]
pub fn skia_color_space_is_srgb(color_space: BorrowedPtr<ColorSpace>) -> bool {
    color_space
        .with_ref_ok(|color_space| color_space.is_srgb())
        .or_log(false)
}

#[unsafe(no_mangle)]
pub fn skia_color_space_drop(ptr: OwnedPtr<ColorSpace>) {
    drop(ptr);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_space_new_srgb() {
        let color_space = skia_color_space_new_srgb();
        assert_eq!(color_space.is_null(), false);
        assert_eq!(skia_color_space_is_srgb(color_space), true);
        skia_color_space_drop(color_space);
    }

    #[test]
    fn color_space_is_srgb_for_null() {
        let color_space: BorrowedPtr<ColorSpace> = std::ptr::null_mut();
        assert_eq!(color_space.is_null(), true);
        assert_eq!(skia_color_space_is_srgb(color_space), false);
        skia_color_space_drop(color_space);
    }
}
