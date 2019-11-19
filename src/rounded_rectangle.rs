use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{RRect, scalar, Rect, Vector, RRectType};

#[no_mangle]
pub fn skia_rounded_rectangle_default() -> *mut ValueBox<RRect> {
    ValueBox::new(RRect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rounded_rectangle_new_radii(
        left: scalar, top: scalar, right: scalar, bottom: scalar,
        r_left_x: scalar, r_left_y: scalar,
        r_top_x: scalar, r_top_y: scalar,
        r_right_x: scalar, r_right_y: scalar,
        r_bottom_x: scalar, r_bottom_y: scalar) -> *mut ValueBox<RRect> {
    let rect = Rect::new(left, top, right, bottom);
    let radii = [
        Vector::new(r_left_x, r_left_y),
        Vector::new(r_top_x, r_top_y),
        Vector::new(r_right_x, r_right_y),
        Vector::new(r_bottom_x, r_bottom_y)];

    ValueBox::new(RRect::new_rect_radii(rect, &radii)).into_raw()
}

#[no_mangle]
pub fn skia_rounded_rectangle_get_type(_ptr: *mut ValueBox<RRect>) -> RRectType {
    _ptr.with(|rounded_rectangle| rounded_rectangle.get_type())
}

#[no_mangle]
pub fn skia_rounded_rectangle_width(_ptr: *mut ValueBox<RRect>) -> scalar {
    _ptr.with(|rounded_rectangle| rounded_rectangle.width())
}

#[no_mangle]
pub fn skia_rounded_rectangle_height(_ptr: *mut ValueBox<RRect>) -> scalar {
    _ptr.with(|rounded_rectangle| rounded_rectangle.height())
}

#[no_mangle]
pub fn skia_rounded_rectangle_set_rect(_rounded_rectangle_ptr: *mut ValueBox<RRect>, _rectangle_ptr: *mut ValueBox<Rect>) {
    _rounded_rectangle_ptr.with(|rounded_rectangle| {
        _rectangle_ptr.with(|rectangle| {
            rounded_rectangle.set_rect(rectangle);
        })
    });
}

#[no_mangle]
pub fn skia_rounded_rectangle_set_oval(_rounded_rectangle_ptr: *mut ValueBox<RRect>, _oval_ptr: *mut ValueBox<Rect>) {
    _rounded_rectangle_ptr.with(|rounded_rectangle| {
        _oval_ptr.with(|oval| {
            rounded_rectangle.set_oval(oval);
        })
    });
}

#[no_mangle]
pub fn skia_rounded_rectangle_drop(_ptr: *mut ValueBox<RRect>) {
    _ptr.drop();
}

#[cfg(test)]
mod test {
    use rounded_rectangle::{skia_rounded_rectangle_default, skia_rounded_rectangle_set_rect, skia_rounded_rectangle_width, skia_rounded_rectangle_height};
    use rectangle::{skia_rectangle_f32_set_ltrb, skia_rectangle_f32_default};
    use boxer::boxes::ValueBoxPointer;

    #[test]
    fn set_rect() {
        let rect = skia_rectangle_f32_default();
        skia_rectangle_f32_set_ltrb(rect, 0.0,0.0, 50.0, 50.0);

        let r_rect = skia_rounded_rectangle_default();
        skia_rounded_rectangle_set_rect(r_rect, rect);

        assert_eq!(skia_rounded_rectangle_width(r_rect), 50.0);
        assert_eq!(skia_rounded_rectangle_height(r_rect), 50.0);

        skia_rectangle_f32_set_ltrb(rect, 0.0,0.0, 100.0, 100.0);

        assert_eq!(skia_rounded_rectangle_width(r_rect), 50.0);
        assert_eq!(skia_rounded_rectangle_height(r_rect), 50.0);

        rect.drop();

        assert_eq!(skia_rounded_rectangle_width(r_rect), 50.0);
        assert_eq!(skia_rounded_rectangle_height(r_rect), 50.0);

        r_rect.drop();
    }
}