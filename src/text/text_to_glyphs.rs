use boxer::boxes::{ValueBox, ValueBoxPointer};
use skia_safe::{Font, TextEncoding, Paint, GlyphId, scalar};
use boxer::string::BoxerString;
use boxer::array::BoxerArray;
use rectangle::skia_rectangle_f32_default;
use text::font::skia_font_text_to_glyphs;

pub struct TextToGlyphs {
    pub font: *mut ValueBox<Font>,
    pub text: *mut BoxerString,
    pub encoding: TextEncoding,
    pub paint: *mut ValueBox<Paint>,
    pub glyphs: *mut ValueBox<BoxerArray<GlyphId>>,
    pub left: scalar,
    pub top: scalar,
    pub right: scalar,
    pub bottom: scalar,
    pub advance: scalar,
}

unsafe impl Send for TextToGlyphs {}
unsafe impl Sync for TextToGlyphs {}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_create() -> *mut ValueBox<Vec<TextToGlyphs>> {
    ValueBox::new(vec![]).into_raw()
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_drop(_ptr: *mut ValueBox<Vec<TextToGlyphs>>) {
    _ptr.drop();
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_length(_ptr: *mut ValueBox<Vec<TextToGlyphs>>) -> usize {
    _ptr.with_not_null_return(0, |vector| vector.len())
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_add(
    _vector_ptr: *mut ValueBox<Vec<TextToGlyphs>>,
    _font_ptr: *mut ValueBox<Font>,
    _text_ptr: *mut BoxerString,
    _encoding: TextEncoding,
    _glyphs_ptr: *mut ValueBox<BoxerArray<GlyphId>>,
    _paint_ptr: *mut ValueBox<Paint>) {
        _vector_ptr.with_not_null(|vector| {
            vector.push(TextToGlyphs {
                font: _font_ptr,
                text: _text_ptr,
                encoding: _encoding,
                paint: _paint_ptr,
                glyphs: _glyphs_ptr,
                left: 0.0,
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                advance: 0.0
            })
        })
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_get_advance_at(_vector_ptr: *mut ValueBox<Vec<TextToGlyphs>>, index: usize) -> scalar {
    _vector_ptr.with_not_null_return(0.0, |vector| { vector[index].advance })
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_get_left_at(_vector_ptr: *mut ValueBox<Vec<TextToGlyphs>>, index: usize) -> scalar {
    _vector_ptr.with_not_null_return(0.0, |vector| { vector[index].left })
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_get_top_at(_vector_ptr: *mut ValueBox<Vec<TextToGlyphs>>, index: usize) -> scalar {
    _vector_ptr.with_not_null_return(0.0, |vector| { vector[index].top })
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_get_right_at(_vector_ptr: *mut ValueBox<Vec<TextToGlyphs>>, index: usize) -> scalar {
    _vector_ptr.with_not_null_return(0.0, |vector| { vector[index].right })
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_get_bottom_at(_vector_ptr: *mut ValueBox<Vec<TextToGlyphs>>, index: usize) -> scalar {
    _vector_ptr.with_not_null_return(0.0, |vector| { vector[index].bottom })
}

#[test]
fn number_of_cpus() {
    println!("{}", num_cpus::get());
}

#[no_mangle]
pub fn skia_text_to_glyphs_batch_process(_texts_ptr: *mut ValueBox<Vec<TextToGlyphs>>) {
    _texts_ptr.with_not_null(|texts| {
        let threads = num_cpus::get() * 4;
        let chunk_size = texts.len() / threads + if texts.len() % threads != 0 { 1 } else { 0 };

        let _ = crossbeam::scope(|scope| {
            // Chop `table` into disjoint sub-slices.
            for each_chunk in texts.as_mut_slice().chunks_mut(chunk_size) {
                // Spawn a thread operating on that subslice.
                scope.spawn(move |_| {
                    for request in each_chunk {
                        let mut bounds = skia_rectangle_f32_default();
                        let advance = skia_font_text_to_glyphs(
                            request.font,
                            request.text,
                            request.encoding,
                            request.glyphs,
                            request.paint,
                            bounds,
                        );
                        bounds.with_value_consumed(|rect| {
                            request.top = rect.top;
                            request.right = rect.right;
                            request.bottom = rect.bottom;
                            request.left = rect.left;
                        });
                        bounds.drop();
                        request.advance = advance;
                    }
                });
            }
            // `crossbeam::scope` ensures that *all* spawned threads join before
            // returning control back from this closure.
        });
    })
}