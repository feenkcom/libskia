use boxer::boxes::{ValueBox, ValueBoxPointer, ReferenceBox, ReferenceBoxPointer};
use skia_safe::textlayout::{ParagraphBuilder, ParagraphStyle, FontCollection, Paragraph, TextStyle, PlaceholderStyle};
use boxer::string::{BoxerString, BoxerStringPointer};
use skia_safe::Canvas;

#[no_mangle]
pub fn skia_paragraph_builder_new(paragraph_style_ptr: *mut ValueBox<ParagraphStyle>, mut font_collection_ptr: *mut ValueBox<FontCollection>) -> *mut ValueBox<ParagraphBuilder> {
    paragraph_style_ptr.with_not_null_return(std::ptr::null_mut(), |style| {
        font_collection_ptr.with_not_null_value_return_block(||{ std::ptr::null_mut() }, |font_collection| {
            ValueBox::new(ParagraphBuilder::new(style, font_collection)).into_raw()
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_build(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilder>) -> *mut ValueBox<Paragraph> {
    paragraph_builder_ptr.with_not_null_return(std::ptr::null_mut(), |builder| {
        ValueBox::new(builder.build()).into_raw()
    })
}

/// Add a text to the paragraph by copying it
#[no_mangle]
pub fn skia_paragraph_builder_add_text(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilder>, string_ptr: *mut BoxerString) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        string_ptr.with_not_null(|string| {
            paragraph_builder.add_text(string.to_string());
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_add_placeholder(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilder>, placeholder_ptr: *mut ValueBox<PlaceholderStyle>) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        placeholder_ptr.with_not_null(|placeholder| {
            paragraph_builder.add_placeholder(placeholder);
        })
    })
}


#[no_mangle]
pub fn skia_paragraph_builder_push_style(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilder>, style_ptr: *mut ValueBox<TextStyle>) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        style_ptr.with_not_null(|style| {
            paragraph_builder.push_style(style);
        })
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_pop_style(paragraph_builder_ptr: *mut ValueBox<ParagraphBuilder>) {
    paragraph_builder_ptr.with_not_null(|paragraph_builder| {
        paragraph_builder.pop();
    })
}

#[no_mangle]
pub fn skia_paragraph_builder_drop(ptr: *mut ValueBox<ParagraphBuilder>) {
    ptr.drop()
}