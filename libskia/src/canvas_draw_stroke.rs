use float_cmp::ApproxEqUlps;
use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::paint::Style;
use skia_safe::{scalar, Canvas, Color, Paint, RRect, Rect, Vector};

#[no_mangle]
pub fn skia_canvas_stroke_rectangle_with_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    width: scalar,
    antialias: bool,
) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.draw_rect(
            Rect::new(left, top, right, bottom),
            Paint::default()
                .set_color(Color::from_argb(a, r, g, b))
                .set_anti_alias(antialias)
                .set_stroke_width(width)
                .set_style(Style::Stroke),
        );
    });
}

/// I fill a rounded rectangle (each corner radius is different) with a given color
#[no_mangle]
pub fn skia_canvas_stroke_rounded_rectangle_with_color(
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
    width: scalar,
    antialias: bool,
) {
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
                    .set_anti_alias(antialias)
                    .set_stroke_width(width)
                    .set_style(Style::Stroke),
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
                    .set_anti_alias(antialias)
                    .set_stroke_width(width)
                    .set_style(Style::Stroke),
            );
        };
    });
}
