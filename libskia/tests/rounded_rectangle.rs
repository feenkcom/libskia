use Skia::rectangle::skia_rectangle_f32_set_ltrb;
use Skia::rounded_rectangle::{
    skia_rounded_rectangle_height, skia_rounded_rectangle_set_rect, skia_rounded_rectangle_width,
};
use skia_safe::{RRect, Rect};
use value_box::BorrowedPtr;

#[test]
fn set_rect() {
    let mut rect = Rect::default();
    skia_rectangle_f32_set_ltrb(BorrowedPtr::from_mut(&mut rect), 0.0, 0.0, 50.0, 50.0);

    let mut r_rect = RRect::default();
    skia_rounded_rectangle_set_rect(
        BorrowedPtr::from_mut(&mut r_rect),
        BorrowedPtr::from_ref(&rect),
    );

    assert_eq!(
        skia_rounded_rectangle_width(BorrowedPtr::from_ref(&r_rect)),
        50.0
    );
    assert_eq!(
        skia_rounded_rectangle_height(BorrowedPtr::from_ref(&r_rect)),
        50.0
    );

    skia_rectangle_f32_set_ltrb(BorrowedPtr::from_mut(&mut rect), 0.0, 0.0, 100.0, 100.0);

    assert_eq!(
        skia_rounded_rectangle_width(BorrowedPtr::from_ref(&r_rect)),
        50.0
    );
    assert_eq!(
        skia_rounded_rectangle_height(BorrowedPtr::from_ref(&r_rect)),
        50.0
    );
}
