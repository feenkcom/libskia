use array_box::ArrayBox;
use skia_safe::{scalar, Matrix};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_matrix_new_identity() -> OwnedPtr<Matrix> {
    OwnedPtr::new(Matrix::new_identity())
}

#[no_mangle]
pub fn skia_matrix_get_all(
    matrix_ptr: BorrowedPtr<Matrix>,
    mut buffer_ptr: BorrowedPtr<ArrayBox<scalar>>,
) {
    matrix_ptr
        .with_ref(|matrix| {
            buffer_ptr.with_mut_ok(|buffer| {
                let mut members: [scalar; 9] = [0.0; 9];
                matrix.get_9(&mut members);
                buffer.set_array(&members);
            })
        })
        .log()
}

#[no_mangle]
pub fn skia_matrix_set_all(
    mut matrix_ptr: BorrowedPtr<Matrix>,
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
    matrix_ptr
        .with_mut_ok(|matrix| {
            matrix.set_all(
                scale_x, skew_x, trans_x, skew_y, scale_y, trans_y, persp_0, persp_1, persp_2,
            );
        })
        .log();
}

#[no_mangle]
pub fn skia_matrix_drop(mut ptr: OwnedPtr<Matrix>) {
    drop(ptr);
}
