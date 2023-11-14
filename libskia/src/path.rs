use array_box::ArrayBox;
use skia_safe::path_utils::fill_path_with_paint;
use skia_safe::{scalar, Paint, Path, PathFillType, Point, Rect, Vector};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_path_new() -> *mut ValueBox<Path> {
    ValueBox::new(Path::new()).into_raw()
}

#[no_mangle]
pub fn skia_path_get_fill_type(path: *mut ValueBox<Path>) -> PathFillType {
    path.with_ref_ok(|path| path.fill_type())
        .or_log(PathFillType::Winding)
}

#[no_mangle]
pub fn skia_path_set_fill_type(path: *mut ValueBox<Path>, fill_type: PathFillType) {
    path.with_mut_ok(|path| {
        path.set_fill_type(fill_type);
    })
    .log();
}

#[no_mangle]
pub fn skia_path_move_to(path: *mut ValueBox<Path>, x: scalar, y: scalar, is_absolute: bool) {
    path.with_mut_ok(|path| {
        if is_absolute {
            path.move_to(Point::new(x, y));
        } else {
            path.r_move_to(Vector::new(x, y));
        }
    })
    .log();
}

#[no_mangle]
pub fn skia_path_line_to(path: *mut ValueBox<Path>, x: scalar, y: scalar, is_absolute: bool) {
    path.with_mut_ok(|path| {
        if is_absolute {
            path.line_to(Point::new(x, y));
        } else {
            path.r_line_to(Vector::new(x, y));
        }
    })
    .log();
}

#[no_mangle]
pub fn skia_path_quad_to(
    path: *mut ValueBox<Path>,
    x1: scalar,
    y1: scalar,
    x2: scalar,
    y2: scalar,
    is_absolute: bool,
) {
    path.with_mut_ok(|path| {
        if is_absolute {
            path.quad_to(Point::new(x1, y1), Point::new(x2, y2));
        } else {
            path.r_quad_to(Vector::new(x1, y1), Vector::new(x2, y2));
        }
    })
    .log();
}

#[no_mangle]
pub fn skia_path_conic_to(
    path: *mut ValueBox<Path>,
    x1: scalar,
    y1: scalar,
    x2: scalar,
    y2: scalar,
    w: scalar,
    is_absolute: bool,
) {
    path.with_mut_ok(|path| {
        if is_absolute {
            path.conic_to(Point::new(x1, y1), Point::new(x2, y2), w);
        } else {
            path.r_conic_to(Vector::new(x1, y1), Vector::new(x2, y2), w);
        }
    })
    .log();
}

#[no_mangle]
pub fn skia_path_cubic_to(
    path: *mut ValueBox<Path>,
    x1: scalar,
    y1: scalar,
    x2: scalar,
    y2: scalar,
    x3: scalar,
    y3: scalar,
    is_absolute: bool,
) {
    path.with_mut_ok(|path| {
        if is_absolute {
            path.cubic_to(Point::new(x1, y1), Point::new(x2, y2), Point::new(x3, y3));
        } else {
            path.r_cubic_to(
                Vector::new(x1, y1),
                Vector::new(x2, y2),
                Vector::new(x3, y3),
            );
        }
    })
    .log();
}

#[no_mangle]
pub fn skia_path_arc_to(
    path: *mut ValueBox<Path>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    start_angle: scalar,
    sweep_angle: scalar,
    force_move_to: bool,
    is_absolute: bool,
) {
    path.with_mut_ok(|path| {
        let rect = if is_absolute {
            Rect::new(left, top, right, bottom)
        } else {
            let current_point = match path.last_pt() {
                None => Point::new(0.0, 0.0),
                Some(point) => point,
            };
            Rect::new(
                current_point.x + left,
                current_point.y + top,
                current_point.x + right,
                current_point.y + bottom,
            )
        };
        path.arc_to(rect, start_angle, sweep_angle, force_move_to);
    })
    .log();
}

#[no_mangle]
pub fn skia_path_close(path: *mut ValueBox<Path>) {
    path.with_mut_ok(|path| {
        path.close();
    })
    .log();
}

#[no_mangle]
pub fn skia_path_count_points(path: *mut ValueBox<Path>) -> usize {
    path.with_ref_ok(|path| path.count_points()).or_log(0)
}

#[no_mangle]
pub fn skia_path_get_points(
    path: *mut ValueBox<Path>,
    points: *mut ValueBox<ArrayBox<f32>>,
) -> usize {
    path.with_ref(|path| {
        points.with_mut_ok(|points| {
            let mut path_points = vec![Point::default(); path.count_points()];
            let points_count = path.get_points(&mut path_points);

            let mut flattened_points: Vec<f32> = Vec::with_capacity(points_count * 2);
            for point in path_points {
                flattened_points.push(point.x);
                flattened_points.push(point.y);
            }
            points.set_vector(flattened_points);
            points_count
        })
    })
    .or_log(0)
}

#[no_mangle]
pub fn skia_path_get_last_point(path: *mut ValueBox<Path>, point: *mut ValueBox<Point>) -> bool {
    path.with_ref(|path| {
        point.with_mut_ok(|point| match path.last_pt() {
            None => false,
            Some(last_point) => {
                point.set(last_point.x, last_point.y);
                true
            }
        })
    })
    .or_log(false)
}

#[no_mangle]
pub fn skia_path_get_stroke_bounds(
    path: *mut ValueBox<Path>,
    paint: *mut ValueBox<Paint>,
    rect: *mut ValueBox<Rect>,
) {
    path.with_ref(|path| {
        paint.with_ref(|paint| {
            rect.with_mut_ok(|rect| {
                let mut fill_path: Path = Path::new();
                if fill_path_with_paint(path, paint, &mut fill_path, None, None) {
                    let fill_rect = fill_path.compute_tight_bounds();
                    rect.set_ltrb(
                        fill_rect.left,
                        fill_rect.top,
                        fill_rect.right,
                        fill_rect.bottom,
                    )
                }
            })
        })
    })
    .log();
}

#[no_mangle]
pub fn skia_path_contains_point(path: *mut ValueBox<Path>, x: f32, y: f32) -> bool {
    path.with_ref_ok(|path| path.contains(Point::new(x, y)))
        .or_log(false)
}

#[no_mangle]
pub fn skia_path_stroke_contains_point(
    path: *mut ValueBox<Path>,
    x: f32,
    y: f32,
    paint_ptr: *mut ValueBox<Paint>,
) -> bool {
    path.with_ref(|path| {
        paint_ptr.with_ref_ok(|paint| {
            let mut fill_path: Path = Path::new();
            if fill_path_with_paint(path, paint, &mut fill_path, None, None) {
                fill_path.contains(Point::new(x, y))
            } else {
                false
            }
        })
    })
    .or_log(false)
}

#[no_mangle]
pub fn skia_path_serialize(path: *mut ValueBox<Path>, data: *mut ValueBox<ArrayBox<u8>>) {
    path.with_ref(|path| {
        data.with_mut_ok(|data| {
            data.set_array(path.serialize().as_bytes());
        })
    })
    .log();
}

#[no_mangle]
pub fn skia_path_drop(path: *mut ValueBox<Path>) {
    path.release();
}
