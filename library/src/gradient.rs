use array_box::ArrayBox;
use std::borrow::Borrow;

use skia_safe::gradient_shader::{Flags, GradientShaderColors};
use skia_safe::{scalar, Color, Matrix, Point, Shader, TileMode};
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn skia_gradient_linear_create(
    from_point_ptr: *mut ValueBox<Point>,
    to_point_ptr: *mut ValueBox<Point>,
    colors_ptr: *mut ValueBox<ArrayBox<Color>>,
    positions_ptr: *mut ValueBox<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    from_point_ptr.with_not_null_value_return(std::ptr::null_mut(), |from_point| {
        to_point_ptr.with_not_null_value_return(std::ptr::null_mut(), |to_point| {
            colors_ptr.with_not_null_return(std::ptr::null_mut(), |colors| {
                positions_ptr.with_not_null_return(std::ptr::null_mut(), |positions| {
                    matrix_ptr.with_not_null_return(std::ptr::null_mut(), |matrix| {
                        match Shader::linear_gradient(
                            (from_point, to_point),
                            GradientShaderColors::Colors(colors.to_slice()),
                            Some(positions.to_slice().borrow()),
                            mode,
                            Flags::from_bits_truncate(bit_flags),
                            Some(matrix.borrow()),
                        ) {
                            None => std::ptr::null_mut(),
                            Some(shader) => ValueBox::new(shader).into_raw(),
                        }
                    })
                })
            })
        })
    })
}

#[no_mangle]
pub fn skia_gradient_radial_create(
    center_ptr: *mut ValueBox<Point>,
    radius: scalar,
    colors_ptr: *mut ValueBox<ArrayBox<Color>>,
    positions_ptr: *mut ValueBox<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    center_ptr.with_not_null_value_return(std::ptr::null_mut(), |center| {
        colors_ptr.with_not_null_return(std::ptr::null_mut(), |colors| {
            positions_ptr.with_not_null_return(std::ptr::null_mut(), |positions| {
                matrix_ptr.with_not_null_return(std::ptr::null_mut(), |matrix| {
                    match Shader::radial_gradient(
                        center,
                        radius,
                        GradientShaderColors::Colors(colors.to_slice()),
                        Some(positions.to_slice().borrow()),
                        mode,
                        Flags::from_bits_truncate(bit_flags),
                        Some(matrix.borrow()),
                    ) {
                        None => std::ptr::null_mut(),
                        Some(shader) => ValueBox::new(shader).into_raw(),
                    }
                })
            })
        })
    })
}

#[no_mangle]
pub fn skia_gradient_two_point_conical_create(
    start_ptr: *mut ValueBox<Point>,
    start_radius: scalar,
    end_ptr: *mut ValueBox<Point>,
    end_radius: scalar,
    colors_ptr: *mut ValueBox<ArrayBox<Color>>,
    positions_ptr: *mut ValueBox<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    start_ptr.with_not_null_value_return(std::ptr::null_mut(), |start| {
        end_ptr.with_not_null_value_return(std::ptr::null_mut(), |end| {
            colors_ptr.with_not_null_return(std::ptr::null_mut(), |colors| {
                positions_ptr.with_not_null_return(std::ptr::null_mut(), |positions| {
                    matrix_ptr.with_not_null_return(std::ptr::null_mut(), |matrix| {
                        match Shader::two_point_conical_gradient(
                            start,
                            start_radius,
                            end,
                            end_radius,
                            GradientShaderColors::Colors(colors.to_slice()),
                            Some(positions.to_slice().borrow()),
                            mode,
                            Flags::from_bits_truncate(bit_flags),
                            Some(matrix.borrow()),
                        ) {
                            None => std::ptr::null_mut(),
                            Some(shader) => ValueBox::new(shader).into_raw(),
                        }
                    })
                })
            })
        })
    })
}

#[no_mangle]
pub fn skia_gradient_sweep_create(
    center_ptr: *mut ValueBox<Point>,
    start_angle: scalar,
    end_angle: scalar,
    colors_ptr: *mut ValueBox<ArrayBox<Color>>,
    positions_ptr: *mut ValueBox<ArrayBox<scalar>>,
    mode: TileMode,
    bit_flags: u32,
    matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    center_ptr.with_not_null_value_return(std::ptr::null_mut(), |center| {
        colors_ptr.with_not_null_return(std::ptr::null_mut(), |colors| {
            positions_ptr.with_not_null_return(std::ptr::null_mut(), |positions| {
                matrix_ptr.with_not_null_return(std::ptr::null_mut(), |matrix| {
                    match Shader::sweep_gradient(
                        center,
                        GradientShaderColors::Colors(colors.to_slice()),
                        Some(positions.to_slice().borrow()),
                        mode,
                        (start_angle, end_angle),
                        Flags::from_bits_truncate(bit_flags),
                        Some(matrix.borrow()),
                    ) {
                        None => std::ptr::null_mut(),
                        Some(shader) => ValueBox::new(shader).into_raw(),
                    }
                })
            })
        })
    })
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
