use skia_safe::{Path, scalar, Point, Vector, PathFillType, Rect};
use boxer::array::BoxerArray;
use boxer::boxes::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_path_new() -> *mut ValueBox<Path> {
    ValueBox::new(Path::new()).into_raw()
}

#[no_mangle]
pub fn skia_path_drop(_ptr: *mut ValueBox<Path>) {
   _ptr.drop();
}

#[no_mangle]
pub fn skia_path_get_fill_type(_path: *mut ValueBox<Path>) -> PathFillType {
    _path.with(|path| path.fill_type())
}

#[no_mangle]
pub fn skia_path_set_fill_type(_path: *mut ValueBox<Path>, fill_type: PathFillType) {
    _path.with(|path| { path.set_fill_type(fill_type); });
}

#[no_mangle]
pub fn skia_path_move_to(_path: *mut ValueBox<Path>, x: scalar, y: scalar, is_absolute: bool) {
    _path.with(|path| {
        if is_absolute {
             path.move_to(Point::new(x,y));
        }
        else {
            path.r_move_to(Vector::new(x,y));
        }
    });
}

#[no_mangle]
pub fn skia_path_line_to(_path: *mut ValueBox<Path>, x: scalar, y: scalar, is_absolute: bool) {
    _path.with(|path| {
        if is_absolute {
             path.line_to(Point::new(x,y));
        }
        else {
            path.r_line_to(Vector::new(x,y));
        }
    });
}

#[no_mangle]
pub fn skia_path_quad_to(_path: *mut ValueBox<Path>, x1: scalar, y1: scalar, x2: scalar, y2: scalar, is_absolute: bool) {
    _path.with(|path| {
        if is_absolute {
             path.quad_to(Point::new(x1,y1), Point::new(x2,y2));
        }
        else {
            path.r_quad_to(Vector::new(x1,y1), Vector::new(x2,y2));
        }
    });
}

#[no_mangle]
pub fn skia_path_conic_to(_path: *mut ValueBox<Path>, x1: scalar, y1: scalar, x2: scalar, y2: scalar, w: scalar, is_absolute: bool) {
    _path.with(|path| {
        if is_absolute {
             path.conic_to(Point::new(x1,y1), Point::new(x2,y2), w);
        }
        else {
            path.r_conic_to(Vector::new(x1,y1), Vector::new(x2,y2), w);
        }
    });
}

#[no_mangle]
pub fn skia_path_cubic_to(_path: *mut ValueBox<Path>, x1: scalar, y1: scalar, x2: scalar, y2: scalar, x3: scalar, y3: scalar, is_absolute: bool) {
    _path.with(|path| {
        if is_absolute {
             path.cubic_to(Point::new(x1,y1), Point::new(x2,y2), Point::new(x3,y3));
        }
        else {
            path.r_cubic_to(Vector::new(x1,y1), Vector::new(x2,y2), Vector::new(x3,y3));
        }
    });
}

#[no_mangle]
pub fn skia_path_arc_to(_path: *mut ValueBox<Path>,
                        left: scalar, top: scalar, right: scalar, bottom: scalar,
                        start_angle: scalar, sweep_angle: scalar,
                        force_move_to: bool, is_absolute: bool) {
    _path.with(|path| {
        let rect = if is_absolute {
            Rect::new(left, top, right, bottom)
        }
        else {
            let current_point = match path.last_pt() {
                None => { Point::new(0.0, 0.0) },
                Some(point) => { point },
            };
            Rect::new(current_point.x + left, current_point.y + top, current_point.x + right, current_point.y + bottom)
        };
        path.arc_to(rect, start_angle, sweep_angle, force_move_to);
    });
}

#[no_mangle]
pub fn skia_path_close(_path: *mut ValueBox<Path>) {
    _path.with(|path| { path.close(); });
}

#[no_mangle]
pub fn skia_path_count_points(_path: *mut ValueBox<Path>) -> usize {
    _path.with(|path| path.count_points())
}

#[no_mangle]
pub fn skia_path_get_points(_path: *mut ValueBox<Path>, _points_ptr: *mut ValueBox<BoxerArray<Point>>) -> usize {
    _path.with(|path| {
        _points_ptr.with(|points| {
            let mut path_points = vec![Point::default(); path.count_points()];
            let points_count = path.get_points(&mut path_points);
            points.set_vector(path_points);
            points_count
        })
    })
}

#[no_mangle]
pub fn skia_path_get_last_point(_path: *mut ValueBox<Path>, _point_ptr: *mut ValueBox<Point>) -> bool {
    _path.with(|path| {
        _point_ptr.with(|point| {
            match path.last_pt() {
                None => { false },
                Some(last_point) => {
                    point.set(last_point.x, last_point.y);
                    true
                },
            }
        })
    })
}

#[no_mangle]
pub fn skia_path_serialize(_path: *mut ValueBox<Path>, _data: *mut ValueBox<BoxerArray<u8>>) {
    _path.with(|path| {
        _data.with(|data| {
            data.set_array(path.serialize().as_bytes());
        })
    });
}
