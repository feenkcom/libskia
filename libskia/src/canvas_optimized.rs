use skia_safe::{Canvas, Color, Paint, Point, TextBlob, scalar};
use value_box::{BorrowedPtr, ReturnBoxerResult};

/// Fill a given text blob with a color and disabled antialias
#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_text_blob_with_color(
    canvas: BorrowedPtr<Canvas>,
    text_blob: BorrowedPtr<TextBlob>,
    x: scalar,
    y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    antialias: bool,
) {
    canvas
        .with_ref_ok(|canvas| {
            text_blob
                .with_ref_ok(|text_blob| {
                    canvas.draw_text_blob(
                        text_blob,
                        Point::new(x, y),
                        Paint::default()
                            .set_color(Color::from_argb(a, r, g, b))
                            .set_anti_alias(antialias),
                    );
                })
                .log();
        })
        .log();
}

/// Fill a given text blob with a color and disabled antialias
#[unsafe(no_mangle)]
pub extern "C" fn skia_canvas_draw_text_blob_with_black_color(
    canvas: BorrowedPtr<Canvas>,
    text_blob: BorrowedPtr<TextBlob>,
    x: scalar,
    y: scalar,
    antialias: bool,
) {
    canvas
        .with_ref_ok(|canvas| {
            text_blob
                .with_ref_ok(|text_blob| {
                    canvas.draw_text_blob(
                        text_blob,
                        Point::new(x, y),
                        Paint::default()
                            .set_color(Color::BLACK)
                            .set_anti_alias(antialias),
                    );
                })
                .log();
        })
        .log();
}
