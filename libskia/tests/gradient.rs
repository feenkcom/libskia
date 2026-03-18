use skia_safe::gradient_shader::{Flags, GradientShaderColors};
use skia_safe::{Color, Point, Shader, TileMode, scalar};

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

    assert!(shader.is_opaque());
    assert!(!shader.is_a_image());
}
