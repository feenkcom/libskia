mod paragraph;

use phlow::{phlow, PhlowObject, PhlowView};
use skia_safe::textlayout::PlaceholderStyle;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[phlow::extensions(SkiaExtensions, PlaceholderStyle)]
impl PlaceholderStyleExtensions {
    #[phlow::view]
    fn info_for(this: &PlaceholderStyle, view: impl PhlowView) -> impl PhlowView {
        view.list()
            .title("Info")
            .priority(5)
            .items(|style: &PlaceholderStyle, object| {
                phlow_all!(vec![
                    ("Width", phlow!(style.width)),
                    ("Height", phlow!(style.height)),
                    ("Alignment", phlow!(style.alignment)),
                    ("Baseline", phlow!(style.baseline)),
                    ("Baseline offset", phlow!(style.baseline_offset)),
                ])
            })
            .item_text(|each: &(&str, PhlowObject), _object| {
                format!("{}: {}", each.0, each.1.to_string())
            })
            .send(|each: &(&str, PhlowObject), object| each.1.clone())
    }
}

#[no_mangle]
pub fn skia_paragraph_placeholder_style_to_phlow(
    placeholder_style: *mut ValueBox<PlaceholderStyle>,
) -> *mut ValueBox<PhlowObject> {
    placeholder_style
        .with_clone(|style| phlow!(style))
        .into_raw()
}
