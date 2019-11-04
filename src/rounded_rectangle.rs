use skia_safe::rrect::{Type, Corner};
use boxer::string::BoxerString;
use boxer::CBox;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{RRect, scalar, Rect};

#[no_mangle]
pub fn skia_rounded_rectangle_type_to_string(_enum: Type, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_rounded_rectangle_corner_to_string(_enum: Corner, _string_ptr: *mut BoxerString) {
    CBox::with_optional_raw(_string_ptr, |option| match option {
        None => {},
        Some(string) => { string.set_string(format!("{:?}", _enum)) },
    })
}

#[no_mangle]
pub fn skia_rounded_rectangle_default() -> *mut ValueBox<RRect> {
    ValueBox::new(RRect::default()).into_raw()
}

#[no_mangle]
pub fn skia_rounded_rectangle_get_type(_ptr: *mut ValueBox<RRect>) -> Type {
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