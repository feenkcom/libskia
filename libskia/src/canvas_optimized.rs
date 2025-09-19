use reference_box::{ReferenceBox, ReferenceBoxPointer};
use skia_safe::{scalar, Canvas, Color, Paint, Point, TextBlob};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

/// Fill a given text blob with a color and disabled antialias
#[no_mangle]
pub fn skia_canvas_draw_text_blob_with_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    text_blob_ptr: *mut ValueBox<TextBlob>,
    x: scalar,
    y: scalar,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    antialias: bool,
) {
    canvas_ptr.with_not_null(|canvas| {
        text_blob_ptr
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
    });
}

/// Fill a given text blob with a color and disabled antialias
#[no_mangle]
pub fn skia_canvas_draw_text_blob_with_black_color(
    canvas_ptr: *mut ReferenceBox<Canvas>,
    text_blob_ptr: *mut ValueBox<TextBlob>,
    x: scalar,
    y: scalar,
    antialias: bool,
) {
    canvas_ptr.with_not_null(|canvas| {
        text_blob_ptr
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
    });
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use skia_safe::{Color, Font, FontStyle, ISize, Paint, Point, Surface, TextBlob, Typeface};

    #[test]
    fn test_text_performance() {
        let width = 1000;
        let height = 1000;

        let mut surface = Surface::new_raster_n32_premul(ISize::new(width, height)).unwrap();
        let canvas = surface.canvas();

        let font = Font::new(
            Typeface::new("Source Sans Pro", FontStyle::normal()).unwrap(),
            Some(14.0),
        );

        let now = Instant::now();
        for _ in 0..25000 {
            let text_blob = TextBlob::from_str("HelloHelloHelloHello", &font).unwrap();
            canvas.draw_text_blob(
                &text_blob,
                Point::new(10.0, 10.0),
                Paint::default()
                    .set_color(Color::BLACK)
                    .set_anti_alias(true),
            );
        }

        let new_now = Instant::now();
        println!("{:?}", new_now.duration_since(now));
    }
}
