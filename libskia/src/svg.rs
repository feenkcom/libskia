use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::{scalar, Canvas, Vector};
use std::error::Error;

use skia_safe::svg::Dom;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub fn skia_svg_parse(svg_string: *mut ValueBox<StringBox>) -> *mut ValueBox<Dom> {
    svg_string
        .with_ref(|svg_string| {
            str::parse::<Dom>(svg_string.as_str())
                .map_err(|error| (Box::new(error) as Box<dyn Error>).into())
        })
        .map(|dom| ValueBox::new(dom))
        .into_raw()
}

#[no_mangle]
pub fn skia_canvas_render_svg(
    canvas: *mut ReferenceBox<Canvas>,
    dom: *mut ValueBox<Dom>,
    x: scalar,
    y: scalar,
) {
    canvas.with_not_null(|canvas| {
        dom.with_ref_ok(|dom| {
            canvas.translate(Vector::new(x, y));
            dom.render(canvas);
            canvas.translate(Vector::new(-x, -y));
        })
        .log();
    });
}
