use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use canvas::assert_canvas;
use float_cmp::ApproxEqUlps;
use skia_safe::{scalar, Canvas, Color, Image, Paint, Point, RRect, Rect, Vector};

#[no_mangle]
pub fn skia_canvas_fill_rectangle_with_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    antialias: bool,
) {
    assert_canvas(canvas_ptr, function!());
    canvas_ptr.with_not_null(|canvas| {
        canvas.draw_rect(
            Rect::new(left, top, right, bottom),
            Paint::default()
                .set_color(Color::from_argb(a, r, g, b))
                .set_anti_alias(antialias),
        );
    });
}

/// I fill a rounded rectangle (each corner radius is different) with a given color
#[no_mangle]
pub fn skia_canvas_fill_rounded_rectangle_with_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_top_left: scalar,
    r_top_right: scalar,
    r_bottom_right: scalar,
    r_bottom_left: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    antialias: bool,
) {
    assert_canvas(canvas_ptr, function!());
    canvas_ptr.with_not_null(|canvas| {
        // if all radii are same we can use a simpler optimized drawing method
        if r_top_left.approx_eq_ulps(&r_top_right, 2)
            && r_top_right.approx_eq_ulps(&r_bottom_right, 2)
            && r_bottom_right.approx_eq_ulps(&r_bottom_left, 2)
            && r_bottom_left.approx_eq_ulps(&r_top_left, 2)
        {
            canvas.draw_round_rect(
                Rect::new(left, top, right, bottom),
                r_top_right,
                r_top_right,
                Paint::default()
                    .set_color(Color::from_argb(a, r, g, b))
                    .set_anti_alias(antialias),
            );
        } else {
            canvas.draw_rrect(
                RRect::new_rect_radii(
                    Rect::new(left, top, right, bottom),
                    &[
                        Vector::new(r_top_left, r_top_left),
                        Vector::new(r_top_right, r_top_right),
                        Vector::new(r_bottom_right, r_bottom_right),
                        Vector::new(r_bottom_left, r_bottom_left),
                    ],
                ),
                Paint::default()
                    .set_color(Color::from_argb(a, r, g, b))
                    .set_anti_alias(antialias),
            );
        };
    });
}

#[no_mangle]
pub fn skia_canvas_fill_image_without_paint(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    image_ptr: *mut ValueBox<Image>,
    x: scalar,
    y: scalar,
) {
    assert_canvas(canvas_ptr, function!());
    canvas_ptr.with_not_null(|canvas| {
        image_ptr.with_not_null(|image| {
            canvas.draw_image(image, Point::new(x, y), None);
        });
    });
}
