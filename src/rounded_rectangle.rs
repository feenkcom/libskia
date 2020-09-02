use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::rrect::Type;
use skia_safe::{scalar, RRect, Rect, Vector};

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
    rounded_rectangle_ptr.with_not_null_return(Type::Empty, |rounded_rectangle| {
        rounded_rectangle.get_type()
    })
}

#[no_mangle]
pub fn skia_rounded_rectangle_width(rounded_rectangle_ptr: *mut ValueBox<RRect>) -> scalar {
    rounded_rectangle_ptr.with_not_null_return(0.0, |rounded_rectangle| rounded_rectangle.width())
}

#[no_mangle]
pub fn skia_rounded_rectangle_height(rounded_rectangle_ptr: *mut ValueBox<RRect>) -> scalar {
    rounded_rectangle_ptr.with_not_null_return(0.0, |rounded_rectangle| rounded_rectangle.height())
}

#[no_mangle]
pub fn skia_rounded_rectangle_set_rect(
    rounded_rectangle_ptr: *mut ValueBox<RRect>,
    rectangle_ptr: *mut ValueBox<Rect>,
) {
    rounded_rectangle_ptr.with_not_null(|rounded_rectangle| {
        rectangle_ptr.with_not_null(|rectangle| {
            rounded_rectangle.set_rect(rectangle);
        })
    });
}

#[no_mangle]
pub fn skia_rounded_rectangle_set_oval(
    rounded_rectangle_ptr: *mut ValueBox<RRect>,
    oval_ptr: *mut ValueBox<Rect>,
) {
    rounded_rectangle_ptr.with_not_null(|rounded_rectangle| {
        oval_ptr.with_not_null(|oval| {
            rounded_rectangle.set_oval(oval);
        })
    });
}

#[no_mangle]
pub fn skia_rounded_rectangle_drop(mut ptr: *mut ValueBox<RRect>) {
    drop!(ptr);
}

#[cfg(test)]
mod test {
    use boxer::boxes::ValueBoxPointer;
    use boxer::ValueBoxPointer;
    use rectangle::{skia_rectangle_f32_default, skia_rectangle_f32_set_ltrb};
    use rounded_rectangle::{
        skia_rounded_rectangle_default, skia_rounded_rectangle_height,
        skia_rounded_rectangle_set_rect, skia_rounded_rectangle_width,
    };

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

        rect.drop();

        assert_eq!(skia_rounded_rectangle_width(r_rect), 50.0);
        assert_eq!(skia_rounded_rectangle_height(r_rect), 50.0);

        r_rect.drop();
    }
}
