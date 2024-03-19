use std::error::Error;
use std::ops::Deref;

use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::svg::Canvas as SvgCanvas;
use skia_safe::svg::Dom;
use skia_safe::svg::Flags as SvgCanvasFlags;
use skia_safe::{scalar, Canvas, Data, Rect, Size, Vector};
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
pub fn skia_svg_set_container_size(dom: *mut ValueBox<Dom>, width: scalar, height: scalar) {
    dom.with_mut_ok(|dom| dom.set_container_size(Size::new(width, height)))
        .log();
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

#[no_mangle]
pub fn skia_svg_dom_drop(ptr: *mut ValueBox<Dom>) {
    ptr.drop();
}

#[no_mangle]
pub fn skia_svg_canvas_new(
    left: scalar,
    top: scalar,
    width: scalar,
    height: scalar,
    flags: u32,
) -> *mut ValueBox<SvgCanvas> {
    let svg_flags = SvgCanvasFlags::from_bits_truncate(flags);
    let canvas = SvgCanvas::new(Rect::from_xywh(left, top, width, height), Some(svg_flags));
    ValueBox::new(canvas).into_raw()
}

#[no_mangle]
pub fn skia_svg_canvas_get_canvas(
    svg_canvas: *mut ValueBox<SvgCanvas>,
) -> *mut ReferenceBox<Canvas> {
    svg_canvas
        .with_ref_ok(|svg_canvas| {
            let canvas = svg_canvas.deref();
            ReferenceBox::new(canvas).into_raw()
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn skia_svg_canvas_end(svg_canvas: *mut ValueBox<SvgCanvas>) -> *mut ValueBox<Data> {
    svg_canvas
        .take_value()
        .map(|mut svg_canvas| ValueBox::new(svg_canvas.end()))
        .into_raw()
}

#[no_mangle]
pub fn skia_svg_canvas_drop(svg_canvas: *mut ValueBox<SvgCanvas>) {
    svg_canvas.release();
}
