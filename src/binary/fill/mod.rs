use binary::command::Command;
use binary::context::Context;
use binary::fill::fill_rectangle::FillRectangle;
use binary::paint::new_paint::NewPaint;
use binary::paint::pop_paint::PopPaint;
use binary::paint::set_rgba::PaintSetRgba;

pub mod fill_rectangle;

#[test]
pub fn test_fill_rectangle_with_color() {
    let mut context = Context::new();

    let mut buffer = vec![];
    NewPaint::new().store_on(&mut buffer);
    PaintSetRgba::new(255, 0, 255, 255).store_on(&mut buffer);
    FillRectangle::new(10.0, 20.0, 30.0, 40.0).store_on(&mut buffer);
    PopPaint::new().store_on(&mut buffer);

    context.execute(buffer.as_slice());

    dbg!(buffer);
}
