use skia_safe::rrect::Type;
use skia_safe::{scalar, RRect, Rect, Vector};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_rounded_rectangle_default() -> *mut ValueBox<RRect> {
    ValueBox::new(RRect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rounded_rectangle_new_radii(
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_left_x: scalar,
    r_left_y: scalar,
    r_top_x: scalar,
    r_top_y: scalar,
    r_right_x: scalar,
    r_right_y: scalar,
    r_bottom_x: scalar,
    r_bottom_y: scalar,
) -> *mut ValueBox<RRect> {
    let rect = Rect::new(left, top, right, bottom);
    let radii = [
        Vector::new(r_left_x, r_left_y),
        Vector::new(r_top_x, r_top_y),
        Vector::new(r_right_x, r_right_y),
        Vector::new(r_bottom_x, r_bottom_y),
    ];

    ValueBox::new(RRect::new_rect_radii(rect, &radii)).into_raw()
}

#[no_mangle]
pub fn skia_rounded_rectangle_get_type(rounded_rectangle_ptr: *mut ValueBox<RRect>) -> Type {
    rounded_rectangle_ptr
        .with_ref_ok(|rounded_rectangle| rounded_rectangle.get_type())
        .or_log(Type::Empty)
}

#[no_mangle]
pub fn skia_rounded_rectangle_width(rounded_rectangle_ptr: *mut ValueBox<RRect>) -> scalar {
    rounded_rectangle_ptr
        .with_ref_ok(|rounded_rectangle| rounded_rectangle.width())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_rounded_rectangle_height(rounded_rectangle_ptr: *mut ValueBox<RRect>) -> scalar {
    rounded_rectangle_ptr
        .with_ref_ok(|rounded_rectangle| rounded_rectangle.height())
        .or_log(0.0)
}

#[no_mangle]
pub fn skia_rounded_rectangle_set_rect(
    rounded_rectangle_ptr: *mut ValueBox<RRect>,
    rectangle_ptr: *mut ValueBox<Rect>,
) {
    rounded_rectangle_ptr
        .with_mut(|rounded_rectangle| {
            rectangle_ptr.with_ref_ok(|rectangle| {
                rounded_rectangle.set_rect(rectangle);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_rounded_rectangle_set_oval(
    rounded_rectangle_ptr: *mut ValueBox<RRect>,
    oval_ptr: *mut ValueBox<Rect>,
) {
    rounded_rectangle_ptr
        .with_mut(|rounded_rectangle| {
            oval_ptr.with_ref_ok(|oval| {
                rounded_rectangle.set_oval(oval);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_rounded_rectangle_drop(ptr: *mut ValueBox<RRect>) {
    ptr.release();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rectangle::{skia_rectangle_f32_default, skia_rectangle_f32_set_ltrb};

    #[test]
    fn set_rect() {
        let mut rect = skia_rectangle_f32_default();
        skia_rectangle_f32_set_ltrb(rect, 0.0, 0.0, 50.0, 50.0);

        let mut r_rect = skia_rounded_rectangle_default();
        skia_rounded_rectangle_set_rect(r_rect, rect);

        assert_eq!(skia_rounded_rectangle_width(r_rect), 50.0);
        assert_eq!(skia_rounded_rectangle_height(r_rect), 50.0);

        skia_rectangle_f32_set_ltrb(rect, 0.0, 0.0, 100.0, 100.0);

        assert_eq!(skia_rounded_rectangle_width(r_rect), 50.0);
        assert_eq!(skia_rounded_rectangle_height(r_rect), 50.0);

        rect.release();

        assert_eq!(skia_rounded_rectangle_width(r_rect), 50.0);
        assert_eq!(skia_rounded_rectangle_height(r_rect), 50.0);

        r_rect.release();
    }
}
