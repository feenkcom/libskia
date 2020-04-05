use binary::command::Command;
use binary::context::Context;
use binary::path::close::Close;
use binary::path::line_to::LineTo;
use binary::path::move_to::MoveTo;
use binary::path::new_path::NewPath;

pub mod close;
pub mod get_path;
pub mod line_to;
pub mod move_to;
pub mod new_path;

#[test]
pub fn test_path() {
    let mut context = Context::new();

    let mut buffer = vec![];
    NewPath::new().store_on(&mut buffer);
    MoveTo::absolute(10.0, 10.0).store_on(&mut buffer);
    LineTo::absolute(20.0, 10.0).store_on(&mut buffer);
    LineTo::absolute(20.0, 20.0).store_on(&mut buffer);
    LineTo::absolute(10.0, 20.0).store_on(&mut buffer);
    Close::new().store_on(&mut buffer);

    context.execute(buffer.as_slice());

    let path = context.peek_path_mut();
    assert_eq!(path.to_svg(), "M10 10L20 10L20 20L10 20L10 10Z");
    dbg!(buffer);
}
