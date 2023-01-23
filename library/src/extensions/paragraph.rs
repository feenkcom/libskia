use phlow::{PhlowObject, PhlowView};
use skia_safe::textlayout::{Paragraph, PlaceholderStyle};
use std::ops::Deref;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use crate::paragraph::paragraph::{ParagraphPiece, ParagraphText, ParagraphWithText};

#[phlow::extensions(SkiaExtensions, ParagraphText)]
impl ParagraphTextExtensions {
    #[phlow::view]
    fn info_for(_this: &ParagraphText, view: impl PhlowView) -> impl PhlowView {
        view.columned_list()
            .title("Info")
            .priority(5)
            .items::<ParagraphText>(|text| {
                phlow_all!(vec![
                    ("Pieces", phlow!(text.pieces.clone(), <ParagraphPiece>)),
                    ("Amount of chars", phlow!(text.char_count())),
                    ("Tab size", phlow!(text.tab_size())),
                ])
            })
            .column_item::<(&str, PhlowObject)>("Property", |each| phlow!(each.0))
            .column_item::<(&str, PhlowObject)>("Value", |each| each.1.clone())
            .send::<(&str, PhlowObject)>(|each| each.1.clone())
    }
}

#[phlow::extensions(SkiaExtensions, ParagraphWithText)]
impl ParagraphWithTextExtensions {
    #[phlow::view]
    fn info_for(_this: &ParagraphWithText, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .priority(5)
            .items::<ParagraphWithText>(|paragraph_with_text| {
                phlow_all!(vec![
                    (
                        "Paragraph",
                        phlow_ref!(
                            paragraph_with_text.paragraph.borrow().deref(),
                            paragraph_with_text.phlow_object()
                        )
                    ),
                    ("Line ranges", phlow!(paragraph_with_text.get_line_ranges())),
                    ("Text", phlow!(paragraph_with_text.text.clone())),
                ])
            })
            .item_text::<(&str, PhlowObject)>(|each| format!("{}: {}", each.0, each.1.to_string()))
            .send::<(&str, PhlowObject)>(|each| each.1.clone())
    }
}

#[phlow::extensions(SkiaExtensions, Paragraph)]
impl ParagraphExtensions {
    #[phlow::view]
    fn info_for(_this: &Paragraph, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .priority(5)
            .items::<Paragraph>(|paragraph| {
                phlow_all!(vec![
                    ("Max width", phlow!(paragraph.max_width())),
                    ("Height", phlow!(paragraph.height())),
                    (
                        "Ideographic baseline",
                        phlow!(paragraph.ideographic_baseline())
                    ),
                    ("Longest line width", phlow!(paragraph.longest_line())),
                    (
                        "Did exceed max lines",
                        phlow!(paragraph.did_exceed_max_lines())
                    ),
                    ("Line number", phlow!(paragraph.line_number())),
                ])
            })
            .item_text::<(&str, PhlowObject)>(|each| format!("{}: {}", each.0, each.1.to_string()))
            .send::<(&str, PhlowObject)>(|each| each.1.clone())
    }
}

#[phlow::extensions(SkiaExtensions, ParagraphPiece)]
impl ParagraphPieceExtensions {
    #[phlow::view]
    fn info_for(_this: &ParagraphPiece, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .priority(5)
            .items::<ParagraphPiece>(|paragraph_piece| match paragraph_piece.deref() {
                ParagraphPiece::Text(text) => {
                    phlow_all!(vec![
                        ("Type", phlow!("Text".to_string())),
                        ("String", phlow!(text.to_string()))
                    ])
                }
                ParagraphPiece::Placeholder(style, char_length) => {
                    phlow_all!(vec![
                        ("Type", phlow!("Placeholder".to_string())),
                        ("Char length", phlow!(char_length.clone())),
                        ("Placeholder style", phlow!(style.clone())),
                    ])
                }
            })
            .item_text::<(&str, PhlowObject)>(|each| format!("{}: {}", each.0, each.1.to_string()))
            .send::<(&str, PhlowObject)>(|each| each.1.clone())
    }
}

#[phlow::extensions(SkiaExtensions, PlaceholderStyle)]
impl PlaceholderStyleExtensions {
    #[phlow::view]
    fn info_for(_this: &PlaceholderStyle, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .priority(5)
            .items::<PlaceholderStyle>(|style| {
                phlow_all!(vec![
                    ("Width", phlow!(style.width)),
                    ("Height", phlow!(style.height)),
                    ("Alignment", phlow!(style.alignment)),
                    ("Baseline", phlow!(style.baseline)),
                    ("Baseline offset", phlow!(style.baseline_offset)),
                ])
            })
            .item_text::<(&str, PhlowObject)>(|each| format!("{}: {}", each.0, each.1.to_string()))
            .send::<(&str, PhlowObject)>(|each| each.1.clone())
    }
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_to_phlow(
    placeholder_style: *mut ValueBox<PlaceholderStyle>,
) -> *mut ValueBox<PhlowObject> {
    placeholder_style
        .with_clone_ok(|style| phlow!(style))
        .into_raw()
}

#[no_mangle]
pub fn skia_paragraph_with_text_to_phlow(
    paragraph: *mut ValueBox<ParagraphWithText>,
) -> *mut ValueBox<PhlowObject> {
    paragraph
        .with_clone_ok(|paragraph| phlow!(paragraph))
        .into_raw()
}
