use crate::paragraph::paragraph::{CharLength, ParagraphText, ParagraphWithText, TabSize};
use skia_safe::textlayout::{
    FontCollection, ParagraphBuilder, ParagraphStyle, PlaceholderStyle, TextStyle,
};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

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
    paragraph_style_ptr: *mut ValueBox<ParagraphStyle>,
    font_collection_ptr: *mut ValueBox<FontCollection>,
    tab_size: TabSize,
) -> *mut ValueBox<ParagraphBuilderWithText> {
    paragraph_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        font_collection_ptr.with_not_null_value_return(std::ptr::null_mut(), |font_collection| {
            ValueBox::new(ParagraphBuilderWithText::new(
                style,
                font_collection,
                tab_size,
            ))
            .into_raw()
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_build(
    paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>,
) -> *mut ValueBox<ParagraphWithText> {
    paragraph_builder_ptr
        .take_value()
        .map(|builder| builder.build())
        .into_raw()
}

/// Add a text to the paragraph by copying it
#[no_mangle]
pub fn skia_paragraph_builder_add_text(
    paragraph_builder: *mut ValueBox<ParagraphBuilderWithText>,
    string: *mut ValueBox<StringBox>,
) {
    paragraph_builder
        .with_mut(|paragraph_builder| {
            string.take_value().map(|string| {
                paragraph_builder.add_text(string);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_builder_add_placeholder(
    paragraph_builder: *mut ValueBox<ParagraphBuilderWithText>,
    placeholder: *mut ValueBox<PlaceholderStyle>,
    char_length: CharLength,
) {
    paragraph_builder
        .with_mut(|paragraph_builder| {
            placeholder.take_value().map(|placeholder| {
                paragraph_builder.add_placeholder(placeholder, char_length);
            })
        })
        .log();
}

#[no_mangle]
pub fn skia_paragraph_builder_push_style(
    paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>,
    style_ptr: *mut ValueBox<TextStyle>,
) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        style_ptr.with_not_null(|style| {
            paragraph_builder.push_style(style);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_pop_style(
    paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>,
) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        paragraph_builder.pop_style();
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_drop(ptr: *mut ValueBox<ParagraphBuilderWithText>) {
    ptr.release();
}
