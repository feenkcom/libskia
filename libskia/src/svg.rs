use std::error::Error;
use std::ops::Deref;

use skia_safe::svg::Canvas as SvgCanvas;
use skia_safe::svg::Dom;
use skia_safe::svg::canvas::Flags as SvgCanvasFlags;
use skia_safe::{Canvas, FontMgr, Rect, Size, Vector, scalar};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_svg_parse(
    svg_string: BorrowedPtr<StringBox>,
    font_mgr: BorrowedPtr<FontMgr>,
) -> OwnedPtr<Dom> {
    svg_string
        .with_ref(|svg_string| {
            font_mgr.with_clone(|font_mgr| {
                Dom::from_str(svg_string.as_str(), font_mgr)
                    .map_err(|error| (Box::new(error) as Box<dyn Error>).into())
            })
        })
        .map(|dom| OwnedPtr::new(dom))
        .or_log(OwnedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_svg_set_container_size(
    mut dom: BorrowedPtr<Dom>,
    width: scalar,
    height: scalar,
) {
    dom.with_mut_ok(|dom| dom.set_container_size(Size::new(width, height)))
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_render_svg(
    canvas: BorrowedPtr<Canvas>,
    dom: BorrowedPtr<Dom>,
    x: scalar,
    y: scalar,
) {
    canvas
        .with_ref_ok(|canvas| {
            dom.with_ref_ok(|dom| {
                canvas.translate(Vector::new(x, y));
                dom.render(canvas);
                canvas.translate(Vector::new(-x, -y));
            })
            .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_svg_dom_drop(ptr: OwnedPtr<Dom>) {
    drop(ptr);
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_svg_canvas_new(
    left: scalar,
    top: scalar,
    width: scalar,
    height: scalar,
    flags: u32,
) -> OwnedPtr<SvgCanvas> {
    let svg_flags = SvgCanvasFlags::from_bits_truncate(flags);
    let canvas = SvgCanvas::new(Rect::from_xywh(left, top, width, height), Some(svg_flags));
    OwnedPtr::new(canvas)
}

/// # Safety
///
/// The returned [`BorrowedPtr<Canvas>`] is borrowed from `svg_canvas` and must
/// not outlive that `SvgCanvas`.
#[unsafe(no_mangle)]
pub extern "C" fn skia_svg_canvas_get_canvas(
    svg_canvas: BorrowedPtr<SvgCanvas>,
) -> BorrowedPtr<Canvas> {
    svg_canvas
        .with_ref_ok(|svg_canvas| BorrowedPtr::from_ref(svg_canvas.deref()))
        .or_log(BorrowedPtr::null())
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_svg_canvas_end(
    svg_canvas: OwnedPtr<SvgCanvas>,
    mut data: BorrowedPtr<StringBox>,
) {
    svg_canvas
        .with_value_ok(|svg_canvas| {
            data.with_mut_ok(|data| {
                let svg = svg_canvas.end();
                let string = std::str::from_utf8(svg.as_bytes()).unwrap();
                data.set_string(string.to_string());
            })
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_svg_canvas_drop(svg_canvas: OwnedPtr<SvgCanvas>) {
    drop(svg_canvas);
}
