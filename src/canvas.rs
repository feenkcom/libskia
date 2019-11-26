use boxer::array::BoxerArray;
use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use layer::SaveLayerRecWrapper;
use skia_safe::canvas::PointMode;
use skia_safe::utils::shadow_utils::ShadowFlags;
use skia_safe::{
    scalar, BlendMode, Canvas, Color, Image, Matrix, Paint, Path, Point, Point3, RRect, Rect,
    SaveLayerRec, TextBlob, Vector,
};
use float_cmp::ApproxEqUlps;

#[inline]
pub fn assert_canvas(canvas_ptr: *mut ReferenceBox<Canvas>) {
    if cfg!(debug_assertions) {
        assert!(
            !canvas_ptr.is_null(),
            "ReferenceBox<Canvas> pointer should not be nil"
        );
        let reference_box = unsafe { boxer::boxes::from_raw(canvas_ptr) };
        let pointer = reference_box.boxed();
        boxer::boxes::into_raw(reference_box);
        assert!(!pointer.is_null(), "Canvas pointer should not be nil");
    }
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
    canvas_ptr.with(|canvas| {
        canvas.draw_color(Color::from_argb(a, r, g, b), blend_mode);
    });
}

#[no_mangle]
pub fn skia_canvas_draw_paint(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_paint(paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_points(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    point_mode: PointMode,
    points_ptr: *mut ValueBox<BoxerArray<Point>>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            points_ptr.with(|points| {
                canvas.draw_points(point_mode, points.to_slice(), paint);
            })
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_point(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    x: scalar,
    y: scalar,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_point(Point::new(x, y), paint);
        });
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
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_line(Point::new(from_x, from_y), Point::new(to_x, to_y), paint);
        });
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
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_rect(Rect::new(left, top, right, bottom), paint);
        });
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
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_oval(Rect::new(left, top, right, bottom), paint);
        });
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
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            canvas.draw_circle(Point::new(center_x, center_y), radius, paint);
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rrect(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    rrect_ptr: *mut ValueBox<RRect>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with(|canvas| {
        rrect_ptr.with(|rrect| {
            paint_ptr.with(|paint| {
                canvas.draw_rrect(rrect, paint);
            });
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_rounded_rectangle (
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
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            // if all radii are same we can use a simpler optimized drawing method
        if r_top_left.approx_eq_ulps(&r_top_right, 2)
            && r_top_right.approx_eq_ulps(&r_bottom_right, 2)
            && r_bottom_right.approx_eq_ulps(&r_bottom_left,2)
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
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_path(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    path_ptr: *mut ValueBox<Path>,
    paint_ptr: *mut ValueBox<Paint>,
) {
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            path_ptr.with(|path| {
                canvas.draw_path(path, paint);
            })
        });
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
    canvas_ptr.with(|canvas| {
        paint_ptr.with(|paint| {
            // text blob can be nil if it was created from an empty string
            text_blob_ptr.with_not_null(|text_blob| {
                canvas.draw_text_blob(text_blob, Point::new(x, y), paint);
            })
        });
    });
}

#[no_mangle]
pub fn skia_canvas_draw_shadow(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    _path_ptr: *mut ValueBox<Path>,
    _z_plane_ptr: *mut ValueBox<Point3>,
    _light_pos_ptr: *mut ValueBox<Point3>,
    light_radius: scalar,
    _ambient_color_ptr: *mut ValueBox<Color>,
    _spot_color_ptr: *mut ValueBox<Color>,
    bit_flags: u32,
) {
    canvas_ptr.with(|canvas| {
        _path_ptr.with_not_null(|path| {
            _z_plane_ptr.with_value(|z_plane| {
                _light_pos_ptr.with_value(|light_pos| {
                    _ambient_color_ptr.with_value(|ambient_color| {
                        _spot_color_ptr.with_value(|spot_color| {
                            canvas.draw_shadow(
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
    canvas_ptr.with(|canvas| {
        image_ptr.with_not_null(|image| match paint_ptr.as_option() {
            None => {
                canvas.draw_image(image, Point::new(x, y), None);
            }
            Some(_paint_ptr) => _paint_ptr.with(|paint| {
                canvas.draw_image(image, Point::new(x, y), Some(paint));
            }),
        });
    });
}

#[no_mangle]
pub fn skia_canvas_translate(canvas_ptr: *mut ReferenceBox<Canvas>, x: scalar, y: scalar) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.translate(Vector::new(x, y));
    });
}

#[no_mangle]
pub fn skia_canvas_scale(canvas_ptr: *mut ReferenceBox<Canvas>, sx: scalar, sy: scalar) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
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
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.rotate(degrees, Some(Point::new(x, y)));
    });
}

#[no_mangle]
pub fn skia_canvas_skew(canvas_ptr: *mut ReferenceBox<Canvas>, sx: scalar, sy: scalar) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.skew((sx, sy));
    });
}

#[no_mangle]
pub fn skia_canvas_concat_matrix(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    matrix_ptr: *mut ValueBox<Matrix>,
) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        matrix_ptr.with(|matrix| {
            canvas.concat(matrix);
        })
    });
}

#[no_mangle]
pub fn skia_canvas_set_matrix(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    matrix_ptr: *mut ValueBox<Matrix>,
) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        matrix_ptr.with(|matrix| {
            canvas.set_matrix(matrix);
        })
    });
}

#[no_mangle]
pub fn skia_canvas_get_matrix(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    matrix_ptr: *mut ValueBox<Matrix>,
) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        matrix_ptr.with(|matrix| {
            let m = canvas.total_matrix();
            let mut buffer: [scalar; 9] = [0.0; 9];
            m.get_9(&mut buffer);
            matrix.set_9(&buffer);
        })
    });
}

#[no_mangle]
pub fn skia_canvas_reset_matrix(canvas_ptr: *mut ReferenceBox<Canvas>) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.reset_matrix();
    })
}

#[no_mangle]
pub fn skia_canvas_flush(canvas_ptr: *mut ReferenceBox<Canvas>) {
    assert_canvas(canvas_ptr);
    canvas_ptr.with(|canvas| {
        canvas.flush();
    })
}

#[no_mangle]
pub fn skia_canvas_save(canvas_ptr: *mut ReferenceBox<Canvas>) -> usize {
    canvas_ptr.with(|canvas| canvas.save())
}

#[no_mangle]
pub fn skia_canvas_save_count(canvas_ptr: *mut ReferenceBox<Canvas>) -> usize {
    canvas_ptr.with(|canvas| canvas.save_count())
}

#[no_mangle]
pub fn skia_canvas_restore(canvas_ptr: *mut ReferenceBox<Canvas>) {
    canvas_ptr.with(|canvas| {
        canvas.restore();
    })
}

#[no_mangle]
pub fn skia_canvas_restore_to_count(canvas_ptr: *mut ReferenceBox<Canvas>, count: usize) {
    canvas_ptr.with(|canvas| {
        canvas.restore_to_count(count);
    })
}

#[no_mangle]
pub fn skia_canvas_save_layer(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    _save_layer_ptr: *mut ValueBox<SaveLayerRecWrapper>,
) -> usize {
    canvas_ptr.with(|canvas| {
        _save_layer_ptr.with(|save_rec| {
            let mut rec: SaveLayerRec = SaveLayerRec::default();
            if save_rec.bounds.is_some() {
                rec = rec.bounds(save_rec.bounds.as_ref().unwrap())
            };
            if save_rec.paint.is_some() {
                rec = rec.paint(save_rec.paint.as_ref().unwrap())
            };
            canvas.save_layer(&rec)
        })
    })
}

#[no_mangle]
pub fn skia_canvas_drop(_ptr: *mut ReferenceBox<Canvas>) {
    _ptr.drop();
}
