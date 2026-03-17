use array_box::ArrayBox;

use skia_safe::gradient_shader::{Flags, GradientShaderColors};
use skia_safe::{scalar, Color, Matrix, Point, Shader, TileMode};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[no_mangle]
pub fn skia_gradient_linear_create(
    from_point_ptr: BorrowedPtr<Point>,
    to_point_ptr: BorrowedPtr<Point>,
    colors_ptr: BorrowedPtr<ArrayBox<Color>>,
    positions_ptr: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    from_point_ptr
        .with_clone(|from_point| {
            to_point_ptr.with_clone(|to_point| {
                colors_ptr.with_ref(|colors| {
                    positions_ptr.with_ref(|positions| {
                        matrix_ptr.with_ref_ok(|matrix| {
                            match Shader::linear_gradient(
                                (from_point, to_point),
                                GradientShaderColors::Colors(colors.to_slice()),
                                Some(positions.to_slice()),
                                mode,
                                Flags::from_bits_truncate(bit_flags),
                                Some(matrix),
                            ) {
                                None => OwnedPtr::null(),
                                Some(shader) => OwnedPtr::new(shader),
                            }
                        })
                    })
                })
            })
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_gradient_radial_create(
    center_ptr: BorrowedPtr<Point>,
    radius: scalar,
    colors_ptr: BorrowedPtr<ArrayBox<Color>>,
    positions_ptr: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    center_ptr
        .with_clone(|center| {
            colors_ptr.with_ref(|colors| {
                positions_ptr.with_ref(|positions| {
                    matrix_ptr.with_ref_ok(|matrix| {
                        match Shader::radial_gradient(
                            center,
                            radius,
                            GradientShaderColors::Colors(colors.to_slice()),
                            Some(positions.to_slice()),
                            mode,
                            Flags::from_bits_truncate(bit_flags),
                            Some(matrix),
                        ) {
                            None => OwnedPtr::null(),
                            Some(shader) => OwnedPtr::new(shader),
                        }
                    })
                })
            })
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_gradient_two_point_conical_create(
    start_ptr: BorrowedPtr<Point>,
    start_radius: scalar,
    end_ptr: BorrowedPtr<Point>,
    end_radius: scalar,
    colors_ptr: BorrowedPtr<ArrayBox<Color>>,
    positions_ptr: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    start_ptr
        .with_clone(|start| {
            end_ptr.with_clone(|end| {
                colors_ptr.with_ref(|colors| {
                    positions_ptr.with_ref(|positions| {
                        matrix_ptr.with_ref_ok(|matrix| {
                            match Shader::two_point_conical_gradient(
                                start,
                                start_radius,
                                end,
                                end_radius,
                                GradientShaderColors::Colors(colors.to_slice()),
                                Some(positions.to_slice()),
                                mode,
                                Flags::from_bits_truncate(bit_flags),
                                Some(matrix),
                            ) {
                                None => OwnedPtr::null(),
                                Some(shader) => OwnedPtr::new(shader),
                            }
                        })
                    })
                })
            })
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_gradient_sweep_create(
    center_ptr: BorrowedPtr<Point>,
    start_angle: scalar,
    end_angle: scalar,
    colors_ptr: BorrowedPtr<ArrayBox<Color>>,
    positions_ptr: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    center_ptr
        .with_clone(|center| {
            colors_ptr.with_ref(|colors| {
                positions_ptr.with_ref(|positions| {
                    matrix_ptr.with_ref_ok(|matrix| {
                        match Shader::sweep_gradient(
                            center,
                            GradientShaderColors::Colors(colors.to_slice()),
                            Some(positions.to_slice()),
                            mode,
                            (start_angle, end_angle),
                            Flags::from_bits_truncate(bit_flags),
                            Some(matrix),
                        ) {
                            None => OwnedPtr::null(),
                            Some(shader) => OwnedPtr::new(shader),
                        }
                    })
                })
            })
        })
        .or_log(OwnedPtr::null())
}

#[cfg(test)]
pub mod test {
    use skia_safe::gradient_shader::{Flags, GradientShaderColors};
    use skia_safe::{scalar, Color, Point, Shader, TileMode};

    #[test]
    fn linear_gradient_lifetime() {
        let from = Point::new(0.0, 0.0);
        let to = Point::new(50.0, 50.0);
        let colors = vec![Color::BLACK, Color::WHITE];
        let positions = vec![0.0 as scalar, 1.0 as scalar];

        let shader = Shader::linear_gradient(
            (from, to),
            GradientShaderColors::Colors(colors.as_slice()),
            positions.as_slice(),
            TileMode::Repeat,
            Flags::INTERPOLATE_COLORS_IN_PREMUL,
            None,
        )
        .unwrap();

        drop(colors);
        drop(positions);

        assert_eq!(shader.is_opaque(), true);
        assert_eq!(shader.is_a_image(), false);
    }
}
