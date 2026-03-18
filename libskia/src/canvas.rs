use array_box::ArrayBox;
use float_cmp::ApproxEqUlps;
use skia_safe::canvas::{PointMode, SaveLayerRec};
use skia_safe::utils::shadow_utils::{ShadowFlags, draw_shadow};
use skia_safe::{
    BlendMode, Canvas, Color, FilterMode, Image, M44, Matrix, MipmapMode, Paint, Path, Point,
    Point3, RRect, Rect, SamplingOptions, TextBlob, Vector, scalar,
};
use value_box::{BorrowedPtr, ReturnBoxerResult};

use crate::layer::SaveLayerRecWrapper;

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_clear(canvas: BorrowedPtr<Canvas>, r: u8, g: u8, b: u8, a: u8) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.clear(Color::from_argb(a, r, g, b));
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_color(
    canvas: BorrowedPtr<Canvas>,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    blend_mode: BlendMode,
) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.draw_color(Color::from_argb(a, r, g, b), blend_mode);
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_paint(canvas: BorrowedPtr<Canvas>, paint: BorrowedPtr<Paint>) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref_ok(|paint| {
                    canvas.draw_paint(paint);
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_points(
    canvas: BorrowedPtr<Canvas>,
    point_mode: PointMode,
    points: BorrowedPtr<ArrayBox<Point>>,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref(|paint| {
                    points.with_ref_ok(|points| {
                        canvas.draw_points(point_mode, points.to_slice(), paint);
                    })
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_point(
    canvas: BorrowedPtr<Canvas>,
    x: scalar,
    y: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref_ok(|paint| {
                    canvas.draw_point(Point::new(x, y), paint);
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_line(
    canvas: BorrowedPtr<Canvas>,
    from_x: scalar,
    from_y: scalar,
    to_x: scalar,
    to_y: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref_ok(|paint| {
                    canvas.draw_line(Point::new(from_x, from_y), Point::new(to_x, to_y), paint);
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_rectangle(
    canvas: BorrowedPtr<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref_ok(|paint| {
                    canvas.draw_rect(Rect::new(left, top, right, bottom), paint);
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_oval(
    canvas: BorrowedPtr<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref_ok(|paint| {
                    canvas.draw_oval(Rect::new(left, top, right, bottom), paint);
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_circle(
    canvas: BorrowedPtr<Canvas>,
    center_x: scalar,
    center_y: scalar,
    radius: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref_ok(|paint| {
                    canvas.draw_circle(Point::new(center_x, center_y), radius, paint);
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_rrect(
    canvas: BorrowedPtr<Canvas>,
    rrect: BorrowedPtr<RRect>,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            rrect
                .with_ref(|rrect| {
                    paint.with_ref_ok(|paint| {
                        canvas.draw_rrect(rrect, paint);
                    })
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_rounded_rectangle(
    canvas: BorrowedPtr<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_top_left: scalar,
    r_top_right: scalar,
    r_bottom_right: scalar,
    r_bottom_left: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref_ok(|paint| {
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
                            paint,
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
                            paint,
                        );
                    };
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_path(
    canvas: BorrowedPtr<Canvas>,
    path: BorrowedPtr<Path>,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref(|paint| {
                    path.with_ref_ok(|path| {
                        canvas.draw_path(path, paint);
                    })
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_text_blob(
    canvas: BorrowedPtr<Canvas>,
    text_blob: BorrowedPtr<TextBlob>,
    x: scalar,
    y: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            paint
                .with_ref(|paint| {
                    // text blob can be nil if it was created from an empty string
                    text_blob.with_ref_ok(|text_blob| {
                        canvas.draw_text_blob(text_blob, Point::new(x, y), paint);
                    })
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_shadow(
    canvas: BorrowedPtr<Canvas>,
    path: BorrowedPtr<Path>,
    z_plane: BorrowedPtr<Point3>,
    light_pos: BorrowedPtr<Point3>,
    light_radius: scalar,
    ambient_color: BorrowedPtr<Color>,
    spot_color: BorrowedPtr<Color>,
    bit_flags: u32,
) {
    let _ = bit_flags;
    canvas
        .with_ref_ok(|canvas| {
            path.with_ref(|path| {
                z_plane.with_clone(|z_plane| {
                    light_pos.with_clone(|light_pos| {
                        ambient_color.with_clone(|ambient_color| {
                            spot_color.with_clone_ok(|spot_color| {
                                draw_shadow(
                                    canvas,
                                    path,
                                    z_plane,
                                    light_pos,
                                    light_radius,
                                    ambient_color,
                                    spot_color,
                                    ShadowFlags::ALL, /*from_bits_truncate(bit_flags)*/
                                );
                            })
                        })
                    })
                })
            })
            .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_image(
    canvas: BorrowedPtr<Canvas>,
    image: BorrowedPtr<Image>, // may be null
    x: scalar,
    y: scalar,
    paint: BorrowedPtr<Paint>,
) {
    canvas
        .with_ref_ok(|canvas| {
            image
                .with_ref(|image| {
                    if !paint.is_null() {
                        paint.with_ref_ok(|paint| {
                            canvas.draw_image_with_sampling_options(
                                image,
                                Point::new(x, y),
                                SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
                                Some(paint),
                            );
                        })
                    } else {
                        canvas.draw_image_with_sampling_options(
                            image,
                            Point::new(x, y),
                            SamplingOptions::new(FilterMode::Linear, MipmapMode::Linear),
                            None,
                        );
                        Ok(())
                    }
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_translate(canvas: BorrowedPtr<Canvas>, x: scalar, y: scalar) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.translate(Vector::new(x, y));
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_scale(canvas: BorrowedPtr<Canvas>, sx: scalar, sy: scalar) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.scale((sx, sy));
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_rotate(
    canvas: BorrowedPtr<Canvas>,
    degrees: scalar,
    x: scalar,
    y: scalar,
) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.rotate(degrees, Some(Point::new(x, y)));
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_skew(canvas: BorrowedPtr<Canvas>, sx: scalar, sy: scalar) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.skew((sx, sy));
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_concat_matrix(
    canvas: BorrowedPtr<Canvas>,
    matrix: BorrowedPtr<Matrix>,
) {
    canvas
        .with_ref_ok(|canvas| {
            matrix
                .with_ref_ok(|matrix| {
                    canvas.concat(matrix);
                })
                .log()
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_set_matrix(canvas: BorrowedPtr<Canvas>, matrix: BorrowedPtr<Matrix>) {
    canvas
        .with_ref_ok(|canvas| {
            matrix
                .with_ref_ok(|matrix| {
                    canvas.set_matrix(&M44::from(matrix));
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_get_matrix(
    canvas: BorrowedPtr<Canvas>,
    mut matrix: BorrowedPtr<Matrix>,
) {
    canvas
        .with_ref_ok(|canvas| {
            matrix
                .with_mut_ok(|matrix| {
                    let m = canvas.local_to_device_as_3x3();
                    let mut buffer: [scalar; 9] = [0.0; 9];
                    m.get_9(&mut buffer);
                    matrix.set_9(&buffer);
                })
                .log();
        })
        .log();
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_reset_matrix(canvas: BorrowedPtr<Canvas>) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.reset_matrix();
        })
        .log()
}

#[unsafe(no_mangle)]
#[deprecated(since = "0.38.0", note = "Replace usage with DirectContext::flush()")]
pub fn skia_canvas_flush(canvas: BorrowedPtr<Canvas>) {
    let _ = canvas;
    eprintln!("skia_canvas_flush is deprecated. Use DirectContext::flush() instead")
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_save(canvas: BorrowedPtr<Canvas>) -> usize {
    canvas.with_ref_ok(|canvas| canvas.save()).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_save_count(canvas: BorrowedPtr<Canvas>) -> usize {
    canvas.with_ref_ok(|canvas| canvas.save_count()).or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_restore(canvas: BorrowedPtr<Canvas>) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.restore();
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_restore_to_count(canvas: BorrowedPtr<Canvas>, count: usize) {
    canvas
        .with_ref_ok(|canvas| {
            canvas.restore_to_count(count);
        })
        .log()
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_save_layer(
    canvas: BorrowedPtr<Canvas>,
    save_layer: BorrowedPtr<SaveLayerRecWrapper>,
) -> usize {
    canvas
        .with_ref_ok(|canvas| {
            save_layer
                .with_ref_ok(|save_rec| {
                    let mut rec: SaveLayerRec = SaveLayerRec::default();
                    if save_rec.bounds.is_some() {
                        rec = rec.bounds(save_rec.bounds.as_ref().unwrap())
                    };
                    if save_rec.paint.is_some() {
                        rec = rec.paint(save_rec.paint.as_ref().unwrap())
                    };
                    canvas.save_layer(&rec)
                })
                .or_log(0)
        })
        .or_log(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_drop(canvas: BorrowedPtr<Canvas>) {
    let _ = canvas;
}
