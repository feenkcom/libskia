use crate::paragraph::paragraph::{CharLength, ParagraphText, ParagraphWithText, TabSize};
use skia_safe::textlayout::{
    FontCollection, ParagraphBuilder, ParagraphStyle, PlaceholderStyle, TextStyle,
};
use string_box::StringBox;
use value_box::{BorrowedPtr, OwnedPtr, ReturnBoxerResult};

pub struct ParagraphBuilderWithText {
    builder: ParagraphBuilder,
    text: ParagraphText,
    tab_size: TabSize,
}

impl ParagraphBuilderWithText {
    pub fn new(
        style: &ParagraphStyle,
        font_collection: impl Into<FontCollection>,
        tab_size: TabSize,
    ) -> Self {
        Self {
            builder: ParagraphBuilder::new(style, font_collection),
            text: ParagraphText::new(tab_size),
            tab_size,
        }
    }

    pub fn add_text(&mut self, string: StringBox) {
        let spaces: String = (0..self.tab_size).map(|_| ' ').collect();

        let replaced_string = string.as_str().replace('\t', &spaces);
        self.builder.add_text(replaced_string.as_str());
        self.text.add_text(string);
    }

    pub fn add_placeholder(
        &mut self,
        placeholder_style: PlaceholderStyle,
        char_length: CharLength,
    ) {
        self.builder.add_placeholder(&placeholder_style);
        self.text.add_placeholder(placeholder_style, char_length);
    }

    pub fn push_style(&mut self, style: &TextStyle) {
        self.builder.push_style(style);
    }

    pub fn pop_style(&mut self) {
        self.builder.pop();
    }

    pub fn build(mut self) -> ParagraphWithText {
        ParagraphWithText::new(self.builder.build(), self.text)
    }
}

#[no_mangle]
pub fn skia_paragraph_builder_new(
    paragraph_style: BorrowedPtr<ParagraphStyle>,
    font_collection: BorrowedPtr<FontCollection>,
    tab_size: TabSize,
) -> OwnedPtr<ParagraphBuilderWithText> {
    paragraph_style
        .with_ref(|style| {
            font_collection.with_ref_ok(|font_collection| {
                OwnedPtr::new(ParagraphBuilderWithText::new(
                    style,
                    font_collection,
                    tab_size,
                ))
            })
        })
        .or_log(OwnedPtr::null())
}

#[no_mangle]
pub fn skia_paragraph_builder_build(
    paragraph_builder_ptr: OwnedPtr<ParagraphBuilderWithText>,
) -> OwnedPtr<ParagraphWithText> {
    paragraph_builder_ptr
        .with_value_ok(|builder| OwnedPtr::new(builder.build()))
        .or_log(OwnedPtr::null())
}

/// Add a text to the paragraph by copying it
#[no_mangle]
pub fn skia_paragraph_builder_add_text(
    mut paragraph_builder: BorrowedPtr<ParagraphBuilderWithText>,
    string: OwnedPtr<StringBox>,
) {
    paragraph_builder
        .with_mut(|paragraph_builder| {
            string.with_value_ok(|string| {
                paragraph_builder.add_text(string);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_builder_add_placeholder(
    mut paragraph_builder: BorrowedPtr<ParagraphBuilderWithText>,
    placeholder: BorrowedPtr<PlaceholderStyle>,
    char_length: CharLength,
) {
    paragraph_builder
        .with_mut(|paragraph_builder| {
            placeholder.with_clone_ok(|placeholder| {
                paragraph_builder.add_placeholder(placeholder, char_length);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_builder_push_style(
    mut paragraph_builder: BorrowedPtr<ParagraphBuilderWithText>,
    style: BorrowedPtr<TextStyle>,
) {
    paragraph_builder
        .with_mut(|paragraph_builder| {
            style.with_ref_ok(|style| {
                paragraph_builder.push_style(style);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_builder_pop_style(
    mut paragraph_builder: BorrowedPtr<ParagraphBuilderWithText>,
) {
    paragraph_builder
        .with_mut_ok(|paragraph_builder| {
            paragraph_builder.pop_style();
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_builder_drop(ptr: OwnedPtr<ParagraphBuilderWithText>) {
    drop(ptr);
}
