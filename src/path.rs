use super::*;
use skia_safe::{Path, scalar, Point, Vector};
use boxer::array::BoxerArray;
use boxer::point::BoxerPointF32;

#[no_mangle]
pub fn skia_path_new() -> *mut Path {
    CBox::into_raw(Path::new())
}

#[no_mangle]
pub fn skia_path_drop(_ptr: *mut Path) {
    CBox::drop(_ptr);
}

#[no_mangle]
pub fn skia_path_move_to(_path: *mut Path, x: scalar, y: scalar, is_absolute: bool) {
    CBox::with_raw(_path, |path|
        if is_absolute {
             path.move_to(Point::new(x,y));
        }
        else {
            path.r_move_to(Vector::new(x,y));
        }
    );
}

#[no_mangle]
pub fn skia_path_line_to(_path: *mut Path, x: scalar, y: scalar, is_absolute: bool) {
    CBox::with_raw(_path, |path|
        if is_absolute {
             path.line_to(Point::new(x,y));
        }
        else {
            path.r_line_to(Vector::new(x,y));
        }
    );
}

#[no_mangle]
pub fn skia_path_quad_to(_path: *mut Path, x1: scalar, y1: scalar, x2: scalar, y2: scalar, is_absolute: bool) {
    CBox::with_raw(_path, |path|
        if is_absolute {
             path.quad_to(Point::new(x1,y1), Point::new(x2,y2));
        }
        else {
            path.r_quad_to(Vector::new(x1,y1), Vector::new(x2,y2));
        }
    );
}

#[no_mangle]
pub fn skia_path_conic_to(_path: *mut Path, x1: scalar, y1: scalar, x2: scalar, y2: scalar, w: scalar, is_absolute: bool) {
    CBox::with_raw(_path, |path|
        if is_absolute {
             path.conic_to(Point::new(x1,y1), Point::new(x2,y2), w);
        }
        else {
            path.r_conic_to(Vector::new(x1,y1), Vector::new(x2,y2), w);
        }
    );
}

#[no_mangle]
pub fn skia_path_cubic_to(_path: *mut Path, x1: scalar, y1: scalar, x2: scalar, y2: scalar, x3: scalar, y3: scalar, is_absolute: bool) {
    CBox::with_raw(_path, |path|
        if is_absolute {
             path.cubic_to(Point::new(x1,y1), Point::new(x2,y2), Point::new(x3,y3));
        }
        else {
            path.r_cubic_to(Vector::new(x1,y1), Vector::new(x2,y2), Vector::new(x3,y3));
        }
    );
}

#[no_mangle]
pub fn skia_path_close(_path: *mut Path) {
    CBox::with_raw(_path, |path| { path.close(); });
}

#[no_mangle]
pub fn skia_path_count_points(_path: *mut Path) -> usize {
    CBox::with_raw(_path, |path| path.count_points())
}

#[no_mangle]
pub fn skia_path_get_points(_path: *mut Path, _points_ptr: *mut BoxerArray<BoxerPointF32>) {
    CBox::with_two_raw(_path, _points_ptr, |path, points| {
        let mut path_points = vec![Point::default(); 0];
        let count_returned = path.get_points(&mut path_points);
        assert_eq!(path_points.len(), count_returned);
        let boxer_points = path_points.into_iter().map(|each| BoxerPointF32::new(each.x, each.y)).collect();
        points.set_vector(boxer_points);
    });
}
