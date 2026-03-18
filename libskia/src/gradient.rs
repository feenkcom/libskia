use array_box::ArrayBox;

use skia_safe::gradient_shader::{Flags, GradientShaderColors};
use skia_safe::{Color, Matrix, Point, Shader, TileMode, scalar};
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

#[unsafe(no_mangle)]
pub extern "C" fn skia_gradient_linear_create(
    from_point: BorrowedPtr<Point>,
    to_point: BorrowedPtr<Point>,
    colors: BorrowedPtr<ArrayBox<Color>>,
    positions: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    from_point
        .with_clone(|from_point| {
            to_point.with_clone(|to_point| {
                colors.with_ref(|colors| {
                    positions.with_ref(|positions| {
                        matrix.with_ref_ok(|matrix| {
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

#[unsafe(no_mangle)]
pub extern "C" fn skia_gradient_radial_create(
    center: BorrowedPtr<Point>,
    radius: scalar,
    colors: BorrowedPtr<ArrayBox<Color>>,
    positions: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    center
        .with_clone(|center| {
            colors.with_ref(|colors| {
                positions.with_ref(|positions| {
                    matrix.with_ref_ok(|matrix| {
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

#[unsafe(no_mangle)]
pub extern "C" fn skia_gradient_two_point_conical_create(
    start: BorrowedPtr<Point>,
    start_radius: scalar,
    end: BorrowedPtr<Point>,
    end_radius: scalar,
    colors: BorrowedPtr<ArrayBox<Color>>,
    positions: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    start
        .with_clone(|start| {
            end.with_clone(|end| {
                colors.with_ref(|colors| {
                    positions.with_ref(|positions| {
                        matrix.with_ref_ok(|matrix| {
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

#[unsafe(no_mangle)]
pub extern "C" fn skia_gradient_sweep_create(
    center: BorrowedPtr<Point>,
    start_angle: scalar,
    end_angle: scalar,
    colors: BorrowedPtr<ArrayBox<Color>>,
    positions: BorrowedPtr<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix: BorrowedPtr<Matrix>,
) -> OwnedPtr<Shader> {
    center
        .with_clone(|center| {
            colors.with_ref(|colors| {
                positions.with_ref(|positions| {
                    matrix.with_ref_ok(|matrix| {
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
