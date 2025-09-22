use array_box::ArrayBox;
use float_cmp::ApproxEqUlps;
use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::canvas::{PointMode, SaveLayerRec};
use skia_safe::utils::shadow_utils::{draw_shadow, ShadowFlags};
use skia_safe::{
    scalar, BlendMode, Canvas, Color, FilterMode, Image, Matrix, MipmapMode, Paint, Path, Point,
    Point3, RRect, Rect, SamplingOptions, TextBlob, Vector, M44,
};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use crate::layer::SaveLayerRecWrapper;

#[no_mangle]
pub fn skia_canvas_clear(canvas_ptr: *mut ReferenceBox<Canvas>, r: u8, g: u8, b: u8, a: u8) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.clear(Color::from_argb(a, r, g, b));
    });
}

#[no_mangle]
pub fn skia_canvas_draw_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    blend_mode: BlendMode,
) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.draw_color(Color::from_argb(a, r, g, b), blend_mode);
    });
}

#[no_mangle]
pub fn skia_canvas_draw_paint(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref_ok(|paint| {
                canvas.draw_paint(paint);
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_points(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    point_mode: PointMode,
    points_ptr: *mut ValueBox<ArrayBox<Point>>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref(|paint| {
                points_ptr.with_ref_ok(|points| {
                    canvas.draw_points(point_mode, points.to_slice(), paint);
                })
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_point(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    x: scalar,
    y: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref_ok(|paint| {
                canvas.draw_point(Point::new(x, y), paint);
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_line(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    from_x: scalar,
    from_y: scalar,
    to_x: scalar,
    to_y: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref_ok(|paint| {
                canvas.draw_line(Point::new(from_x, from_y), Point::new(to_x, to_y), paint);
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rectangle(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref_ok(|paint| {
                canvas.draw_rect(Rect::new(left, top, right, bottom), paint);
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_oval(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref_ok(|paint| {
                canvas.draw_oval(Rect::new(left, top, right, bottom), paint);
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_circle(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    center_x: scalar,
    center_y: scalar,
    radius: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref_ok(|paint| {
                canvas.draw_circle(Point::new(center_x, center_y), radius, paint);
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rrect(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    rrect_ptr: *mut ValueBox<RRect>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        rrect_ptr
            .with_ref(|rrect| {
                paint_ptr.with_ref_ok(|paint| {
                    canvas.draw_rrect(rrect, paint);
                })
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rounded_rectangle(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    r_top_left: scalar,
    r_top_right: scalar,
    r_bottom_right: scalar,
    r_bottom_left: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
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
    });
}

#[no_mangle]
pub fn skia_canvas_draw_path(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    path_ptr: *mut ValueBox<Path>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref(|paint| {
                path_ptr.with_ref_ok(|path| {
                    canvas.draw_path(path, paint);
                })
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_text_blob(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    text_blob_ptr: *mut ValueBox<TextBlob>,
    x: scalar,
    y: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        paint_ptr
            .with_ref(|paint| {
                // text blob can be nil if it was created from an empty string
                text_blob_ptr.with_ref_ok(|text_blob| {
                    canvas.draw_text_blob(text_blob, Point::new(x, y), paint);
                })
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_draw_shadow(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    path_ptr: *mut ValueBox<Path>,
    z_plane_ptr: *mut ValueBox<Point3>,
    light_pos_ptr: *mut ValueBox<Point3>,
    light_radius: scalar,
    ambient_color_ptr: *mut ValueBox<Color>,
    spot_color_ptr: *mut ValueBox<Color>,
    _bit_flags: u32,
) {
    canvas_ptr.with_not_null(|canvas| {
        path_ptr
            .with_ref(|path| {
                z_plane_ptr.with_clone(|z_plane| {
                    light_pos_ptr.with_clone(|light_pos| {
                        ambient_color_ptr.with_clone(|ambient_color| {
                            spot_color_ptr.with_clone_ok(|spot_color| {
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
}

#[no_mangle]
pub fn skia_canvas_draw_image(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    image_ptr: *mut ValueBox<Image>, // may be null
    x: scalar,
    y: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with_not_null(|canvas| {
        image_ptr
            .with_ref(|image| {
                if paint_ptr.has_value() {
                    paint_ptr.with_ref_ok(|paint| {
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
    });
}

#[no_mangle]
pub fn skia_canvas_translate(canvas_ptr: *mut ReferenceBox<Canvas>, x: scalar, y: scalar) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.translate(Vector::new(x, y));
    });
}

#[no_mangle]
pub fn skia_canvas_scale(canvas_ptr: *mut ReferenceBox<Canvas>, sx: scalar, sy: scalar) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.scale((sx, sy));
    });
}

#[no_mangle]
pub fn skia_canvas_rotate(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    degrees: scalar,
    x: scalar,
    y: scalar,
) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.rotate(degrees, Some(Point::new(x, y)));
    });
}

#[no_mangle]
pub fn skia_canvas_skew(canvas_ptr: *mut ReferenceBox<Canvas>, sx: scalar, sy: scalar) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.skew((sx, sy));
    });
}

#[no_mangle]
pub fn skia_canvas_concat_matrix(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    matrix_ptr: *mut ValueBox<Matrix>,
) {
    canvas_ptr.with_not_null(|canvas| {
        matrix_ptr
            .with_ref_ok(|matrix| {
                canvas.concat(matrix);
            })
            .log()
    });
}

#[no_mangle]
pub fn skia_canvas_set_matrix(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    matrix_ptr: *mut ValueBox<Matrix>,
) {
    canvas_ptr.with_not_null(|canvas| {
        matrix_ptr
            .with_ref_ok(|matrix| {
                canvas.set_matrix(&M44::from(matrix));
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_get_matrix(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    matrix_ptr: *mut ValueBox<Matrix>,
) {
    canvas_ptr.with_not_null(|canvas| {
        matrix_ptr
            .with_mut_ok(|matrix| {
                let m = canvas.local_to_device_as_3x3();
                let mut buffer: [scalar; 9] = [0.0; 9];
                m.get_9(&mut buffer);
                matrix.set_9(&buffer);
            })
            .log();
    });
}

#[no_mangle]
pub fn skia_canvas_reset_matrix(canvas_ptr: *mut ReferenceBox<Canvas>) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.reset_matrix();
    })
}

#[no_mangle]
#[deprecated(since = "0.38.0", note = "Replace usage with DirectContext::flush()")]
pub fn skia_canvas_flush(_canvas_ptr: *mut ReferenceBox<Canvas>) {
    eprintln!("skia_canvas_flush is deprecated. Use DirectContext::flush() instead")
}

#[no_mangle]
pub fn skia_canvas_save(canvas_ptr: *mut ReferenceBox<Canvas>) -> usize {
    canvas_ptr.with_not_null_return(0, |canvas| canvas.save())
}

#[no_mangle]
pub fn skia_canvas_save_count(canvas_ptr: *mut ReferenceBox<Canvas>) -> usize {
    canvas_ptr.with_not_null_return(0, |canvas| canvas.save_count())
}

#[no_mangle]
pub fn skia_canvas_restore(canvas_ptr: *mut ReferenceBox<Canvas>) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.restore();
    })
}

#[no_mangle]
pub fn skia_canvas_restore_to_count(canvas_ptr: *mut ReferenceBox<Canvas>, count: usize) {
    canvas_ptr.with_not_null(|canvas| {
        canvas.restore_to_count(count);
    })
}

#[no_mangle]
pub fn skia_canvas_save_layer(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    _save_layer_ptr: *mut ValueBox<SaveLayerRecWrapper>,
) -> usize {
    canvas_ptr.with_not_null_return(0, |canvas| {
        _save_layer_ptr
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
}

#[no_mangle]
pub fn skia_canvas_drop(_ptr: &mut *mut ReferenceBox<Canvas>) {
    (*_ptr).drop();
    *_ptr = std::ptr::null_mut();
}
