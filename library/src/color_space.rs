use skia_safe::ColorSpace;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_color_space_new_srgb() -> *mut ValueBox<ColorSpace> {
    ValueBox::new(ColorSpace::new_srgb()).into_raw()
}

#[no_mangle]
pub fn skia_color_space_is_srgb(color_space: *mut ValueBox<ColorSpace>) -> bool {
    color_space
        .to_ref()
        .map(|color_space| color_space.is_srgb())
        .or_log(false)
}

#[no_mangle]
pub fn skia_color_space_drop(ptr: *mut ValueBox<ColorSpace>) {
    ptr.release();
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
        let color_space: *mut ValueBox<ColorSpace> = std::ptr::null_mut();
        assert_eq!(color_space.is_null(), true);
        assert_eq!(skia_color_space_is_srgb(color_space), false);
        skia_color_space_drop(color_space);
    }
}
