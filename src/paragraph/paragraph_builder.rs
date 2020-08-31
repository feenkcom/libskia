use boxer::boxes::{ValueBox, ValueBoxPointer, ReferenceBox, ReferenceBoxPointer};
use skia_safe::textlayout::{ParagraphBuilder, ParagraphStyle, FontCollection, Paragraph, TextStyle, PlaceholderStyle};
use boxer::string::BoxerString;
use skia_safe::Canvas;
use paragraph::paragraph::{ParagraphText, ParagraphWithText, TabSize};


pub struct ParagraphBuilderWithText {
    builder: ParagraphBuilder,
    text: ParagraphText,
    tab_size: TabSize
}

impl ParagraphBuilderWithText {
    pub fn new(style: &ParagraphStyle, font_collection: impl Into<FontCollection>, tab_size: TabSize) -> Self {
        Self {
            builder: ParagraphBuilder::new(style, font_collection),
            text: ParagraphText::new(tab_size),
            tab_size
        }
    }

    pub fn add_text(&mut self, string: BoxerString) {
        let spaces: String = (0..self.tab_size).map(|i| ' ').collect();

        let replaced_string = string.as_str().replace('\t', &spaces);
        self.builder.add_text(replaced_string.as_str());
        self.text.add_text(string);
    }

    pub fn add_placeholder(&mut self, placeholder_style: PlaceholderStyle) {
        self.builder.add_placeholder(&placeholder_style);
        self.text.add_placeholder(placeholder_style);
    }

    pub fn set_paragraph_style(&mut self, style: &ParagraphStyle) {
        self.builder.set_paragraph_style(style);
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
pub fn skia_paragraph_builder_new(paragraph_style_ptr: *mut ValueBox<ParagraphStyle>, mut font_collection_ptr: *mut ValueBox<FontCollection>, tab_size: TabSize) -> *mut ValueBox<ParagraphBuilderWithText> {
    paragraph_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        font_collection_ptr.with_not_null_value_return_block(||{ std::ptr::null_mut() }, |font_collection| {
            ValueBox::new(ParagraphBuilderWithText::new(style, font_collection, tab_size)).into_raw()
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_build(mut paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>) -> *mut ValueBox<ParagraphWithText> {
    paragraph_builder_ptr.with_value_consumed(|builder| {
        ValueBox::new(builder.build()).into_raw()
    })
}

/// Add a text to the paragraph by copying it
#[no_mangle]
pub fn skia_paragraph_builder_add_text(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>, mut string_ptr: *mut ValueBox<BoxerString>) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        string_ptr.with_not_null_value_consumed(|string| {
            paragraph_builder.add_text(string);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_add_placeholder(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>, mut placeholder_ptr: *mut ValueBox<PlaceholderStyle>) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        placeholder_ptr.with_not_null_value_consumed(|placeholder| {
            paragraph_builder.add_placeholder(placeholder);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_push_style(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>, style_ptr: *mut ValueBox<TextStyle>) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        style_ptr.with_not_null(|style| {
            paragraph_builder.push_style(style);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_pop_style(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilderWithText>) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        paragraph_builder.pop_style();
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_drop(ptr: *mut ValueBox<ParagraphBuilderWithText>) {
    ptr.drop()
}