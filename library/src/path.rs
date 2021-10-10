use boxer::array::BoxerArray;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use skia_safe::{scalar, Paint, Path, PathFillType, Point, Rect, Vector};

#[no_mangle]
pub fn skia_path_new() -> *mut ValueBox<Path> {
    ValueBox::new(Path::new()).into_raw()
}

#[no_mangle]
pub fn skia_path_get_fill_type(path_ptr: *mut ValueBox<Path>) -> PathFillType {
    path_ptr.with_not_null_return(PathFillType::Winding, |path| path.fill_type())
}

#[no_mangle]
pub fn skia_path_set_fill_type(path_ptr: *mut ValueBox<Path>, fill_type: PathFillType) {
    path_ptr.with_not_null(|path| {
        path.set_fill_type(fill_type);
    });
}

#[no_mangle]
pub fn skia_path_move_to(path_ptr: *mut ValueBox<Path>, x: scalar, y: scalar, is_absolute: bool) {
    path_ptr.with_not_null(|path| {
        if is_absolute {
            path.move_to(Point::new(x, y));
        } else {
            path.r_move_to(Vector::new(x, y));
        }
    });
}

#[no_mangle]
pub fn skia_path_line_to(path_ptr: *mut ValueBox<Path>, x: scalar, y: scalar, is_absolute: bool) {
    path_ptr.with_not_null(|path| {
        if is_absolute {
            path.line_to(Point::new(x, y));
        } else {
            path.r_line_to(Vector::new(x, y));
        }
    });
}

#[no_mangle]
pub fn skia_path_quad_to(
    path_ptr: *mut ValueBox<Path>,
    x1: scalar,
    y1: scalar,
    x2: scalar,
    y2: scalar,
    is_absolute: bool,
) {
    path_ptr.with_not_null(|path| {
        if is_absolute {
            path.quad_to(Point::new(x1, y1), Point::new(x2, y2));
        } else {
            path.r_quad_to(Vector::new(x1, y1), Vector::new(x2, y2));
        }
    });
}

#[no_mangle]
pub fn skia_path_conic_to(
    path_ptr: *mut ValueBox<Path>,
    x1: scalar,
    y1: scalar,
    x2: scalar,
    y2: scalar,
    w: scalar,
    is_absolute: bool,
) {
    path_ptr.with_not_null(|path| {
        if is_absolute {
            path.conic_to(Point::new(x1, y1), Point::new(x2, y2), w);
        } else {
            path.r_conic_to(Vector::new(x1, y1), Vector::new(x2, y2), w);
        }
    });
}

#[no_mangle]
pub fn skia_path_cubic_to(
    path_ptr: *mut ValueBox<Path>,
    x1: scalar,
    y1: scalar,
    x2: scalar,
    y2: scalar,
    x3: scalar,
    y3: scalar,
    is_absolute: bool,
) {
    path_ptr.with_not_null(|path| {
        if is_absolute {
            path.cubic_to(Point::new(x1, y1), Point::new(x2, y2), Point::new(x3, y3));
        } else {
            path.r_cubic_to(
                Vector::new(x1, y1),
                Vector::new(x2, y2),
                Vector::new(x3, y3),
            );
        }
    });
}

#[no_mangle]
pub fn skia_path_arc_to(
    path_ptr: *mut ValueBox<Path>,
    left: scalar,
    top: scalar,
    right: scalar,
    bottom: scalar,
    start_angle: scalar,
    sweep_angle: scalar,
    force_move_to: bool,
    is_absolute: bool,
) {
    path_ptr.with_not_null(|path| {
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
    });
}

#[no_mangle]
pub fn skia_path_close(path_ptr: *mut ValueBox<Path>) {
    path_ptr.with_not_null(|path| {
        path.close();
    });
}

#[no_mangle]
pub fn skia_path_count_points(path_ptr: *mut ValueBox<Path>) -> usize {
    path_ptr.with_not_null_return(0, |path| path.count_points())
}

#[no_mangle]
pub fn skia_path_get_points(
    path_ptr: *mut ValueBox<Path>,
    points_ptr: *mut ValueBox<BoxerArray<Point>>,
) -> usize {
    path_ptr.with_not_null_return(0, |path| {
        points_ptr.with_not_null_return(0, |points| {
            let mut path_points = vec![Point::default(); path.count_points()];
            let points_count = path.get_points(&mut path_points);
            points.set_vector(path_points);
            points_count
        })
    })
}

#[no_mangle]
pub fn skia_path_get_last_point(
    path_ptr: *mut ValueBox<Path>,
    point_ptr: *mut ValueBox<Point>,
) -> bool {
    path_ptr.with_not_null_return(false, |path| {
        point_ptr.with_not_null_return(false, |point| match path.last_pt() {
            None => false,
            Some(last_point) => {
                point.set(last_point.x, last_point.y);
                true
            }
        })
    })
}

#[no_mangle]
pub fn skia_path_get_stroke_bounds(
    path_ptr: *mut ValueBox<Path>,
    paint_ptr: *mut ValueBox<Paint>,
    rect_ptr: *mut ValueBox<Rect>,
) {
    path_ptr.with_not_null(|path| {
        paint_ptr.with_not_null(|paint| {
            rect_ptr.with_not_null(|rect| match paint.get_fill_path(path, None, None) {
                None => {}
                Some(fill_path) => {
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
}

#[no_mangle]
pub fn skia_path_contains_point(path_ptr: *mut ValueBox<Path>, x: f32, y: f32) -> bool {
    path_ptr.with_not_null_return(false, |path| path.contains(Point::new(x, y)))
}

#[no_mangle]
pub fn skia_path_stroke_contains_point(
    path_ptr: *mut ValueBox<Path>,
    x: f32,
    y: f32,
    paint_ptr: *mut ValueBox<Paint>,
) -> bool {
    path_ptr.with_not_null_return(false, |path| {
        paint_ptr.with_not_null_return(false, |paint| match paint.get_fill_path(path, None, None) {
            None => false,
            Some(fill_path) => fill_path.contains(Point::new(x, y)),
        })
    })
}

#[no_mangle]
pub fn skia_path_serialize(path_ptr: *mut ValueBox<Path>, data_ptr: *mut ValueBox<BoxerArray<u8>>) {
    path_ptr.with_not_null(|path| {
        data_ptr.with_not_null(|data| {
            data.set_array(path.serialize().as_bytes());
        })
    });
}

#[no_mangle]
pub fn skia_path_drop(ptr: &mut *mut ValueBox<Path>) {
    drop!(ptr);
}
