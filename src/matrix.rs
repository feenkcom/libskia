use boxer::array::BoxerArray;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{scalar, Matrix};

#[no_mangle]
pub fn skia_matrix_new_identity() -> *mut ValueBox<Matrix> {
    ValueBox::new(Matrix::new_identity()).into_raw()
}

#[no_mangle]
pub fn skia_matrix_get_all(
    _ptr: *mut ValueBox<Matrix>,
    _buffer_ptr: *mut ValueBox<BoxerArray<scalar>>,
) {
    _ptr.with_not_null(|matrix| {
        _buffer_ptr.with_not_null(|buffer| {
            let mut members: [scalar; 9] = [0.0; 9];
            matrix.get_9(&mut members);
            buffer.set_array(&members);
        })
    })
}

#[no_mangle]
pub fn skia_matrix_set_all(
    _ptr: *mut ValueBox<Matrix>,
    scale_x: scalar,
    skew_x: scalar,
    trans_x: scalar,
    skew_y: scalar,
    scale_y: scalar,
    trans_y: scalar,
    persp_0: scalar,
    persp_1: scalar,
    persp_2: scalar,
) {
    _ptr.with_not_null(|matrix| {
        matrix.set_all(
            scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, persp_0, persp_1, persp_2,
        );
    });
}

#[no_mangle]
pub fn skia_matrix_drop(_ptr: *mut ValueBox<Matrix>) {
    _ptr.drop();
}
