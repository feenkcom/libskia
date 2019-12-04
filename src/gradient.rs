use boxer::array::BoxerArray;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::gradient_shader::{Flags, GradientShaderColors};
use skia_safe::{scalar, Color, Matrix, Point, Shader, TileMode};
use std::borrow::Borrow;

#[no_mangle]
pub fn skia_gradient_linear_create(
    _from_point_ptr: *mut ValueBox<Point>,
    _to_point_ptr: *mut ValueBox<Point>,
    _colors_ptr: *mut ValueBox<BoxerArray<Color>>,
    _positions_ptr: *mut ValueBox<BoxerArray<scalar>>,
    _mode: TileMode,
    _bit_flags: u32,
    _matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    _from_point_ptr.with_value(|from_point| {
        _to_point_ptr.with_value(|to_point| {
            _colors_ptr.with(|colors| {
                _positions_ptr.with(|positions| {
                    _matrix_ptr.with_reference(|matrix| {
                        match Shader::linear_gradient(
                            (from_point, to_point),
                            GradientShaderColors::Colors(colors.to_slice()),
                            Some(positions.to_slice().borrow()),
                            _mode,
                            Flags::from_bits_truncate(_bit_flags),
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
    _center_ptr: *mut ValueBox<Point>,
    _radius: scalar,
    _colors_ptr: *mut ValueBox<BoxerArray<Color>>,
    _positions_ptr: *mut ValueBox<BoxerArray<scalar>>,
    _mode: TileMode,
    _bit_flags: u32,
    _matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    _center_ptr.with_value(|center| {
        _colors_ptr.with(|colors| {
            _positions_ptr.with(|positions| {
                _matrix_ptr.with_reference(|matrix| {
                    match Shader::radial_gradient(
                        center,
                        _radius,
                        GradientShaderColors::Colors(colors.to_slice()),
                        Some(positions.to_slice().borrow()),
                        _mode,
                        Flags::from_bits_truncate(_bit_flags),
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
    _start_ptr: *mut ValueBox<Point>,
    _start_radius: scalar,
    _end_ptr: *mut ValueBox<Point>,
    _end_radius: scalar,
    _colors_ptr: *mut ValueBox<BoxerArray<Color>>,
    _positions_ptr: *mut ValueBox<BoxerArray<scalar>>,
    _mode: TileMode,
    _bit_flags: u32,
    _matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    _start_ptr.with_value(|start| {
        _end_ptr.with_value(|end| {
            _colors_ptr.with(|colors| {
                _positions_ptr.with(|positions| {
                    _matrix_ptr.with_reference(|matrix| {
                        match Shader::two_point_conical_gradient(
                            start,
                            _start_radius,
                            end,
                            _end_radius,
                            GradientShaderColors::Colors(colors.to_slice()),
                            Some(positions.to_slice().borrow()),
                            _mode,
                            Flags::from_bits_truncate(_bit_flags),
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
    _center_ptr: *mut ValueBox<Point>,
    _start_angle: scalar,
    _end_angle: scalar,
    _colors_ptr: *mut ValueBox<BoxerArray<Color>>,
    _positions_ptr: *mut ValueBox<BoxerArray<scalar>>,
    _mode: TileMode,
    _bit_flags: u32,
    _matrix_ptr: *mut ValueBox<Matrix>,
) -> *mut ValueBox<Shader> {
    _center_ptr.with_value(|center| {
        _colors_ptr.with(|colors| {
            _positions_ptr.with(|positions| {
                _matrix_ptr.with_reference(|matrix| {
                    match Shader::sweep_gradient(
                        center,
                        GradientShaderColors::Colors(colors.to_slice()),
                        Some(positions.to_slice().borrow()),
                        _mode,
                        (_start_angle, _end_angle),
                        Flags::from_bits_truncate(_bit_flags),
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
