use boxer::array::BoxerArray;
use boxer::{ValueBox, ValueBoxPointer};
use skia_safe::Color;

#[no_mangle]
pub fn skia_color_default() -> *mut ValueBox<Color> {
    ValueBox::new(Color::default()).into_raw()
}

#[no_mangle]
pub fn skia_color_create(r: u8, g: u8, b: u8, a: u8) -> *mut ValueBox<Color> {
    ValueBox::new(Color::from_argb(a, r, g, b)).into_raw()
}

#[no_mangle]
pub fn skia_color_create_argb(argb: u32) -> *mut ValueBox<Color> {
    ValueBox::new(Color::new(argb)).into_raw()
}

#[no_mangle]
pub fn skia_color_get_red(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.r())
}

#[no_mangle]
pub fn skia_color_get_green(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.g())
}

#[no_mangle]
pub fn skia_color_get_blue(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.b())
}

#[no_mangle]
pub fn skia_color_get_alpha(color_ptr: *mut ValueBox<Color>) -> u8 {
    color_ptr.with_not_null_return(0, |color| color.a())
}

#[no_mangle]
pub fn skia_color_drop(mut ptr: *mut ValueBox<Color>) {
    ptr.drop();
}

#[no_mangle]
pub fn skia_color_array_default() -> *mut ValueBox<BoxerArray<Color>> {
    ValueBox::new(BoxerArray::new()).into_raw()
}

#[no_mangle]
pub fn skia_color_array_create_with(
    element_ptr: *mut ValueBox<Color>,
    amount: usize,
) -> *mut ValueBox<BoxerArray<Color>> {
    element_ptr.with_not_null_value_return(std::ptr::null_mut(), |color| {
        BoxerArray::<Color>::boxer_array_create_with(color, amount)
    })
}

#[no_mangle]
pub fn skia_color_array_get_length(_ptr: *mut ValueBox<BoxerArray<Color>>) -> usize {
    BoxerArray::<Color>::boxer_array_get_length(_ptr)
}

#[no_mangle]
pub fn skia_color_array_get_capacity(_ptr: *mut ValueBox<BoxerArray<Color>>) -> usize {
    BoxerArray::<Color>::boxer_array_get_capacity(_ptr)
}

#[no_mangle]
pub fn skia_color_array_get_data(_ptr: *mut ValueBox<BoxerArray<Color>>) -> *mut Color {
    BoxerArray::<Color>::boxer_array_get_data(_ptr)
}

#[no_mangle]
pub fn skia_color_array_at(
    array_ptr: *mut ValueBox<BoxerArray<Color>>,
    index: usize,
) -> *mut ValueBox<Color> {
    array_ptr.with_not_null_return(std::ptr::null_mut(), |array| {
        ValueBox::new(array.to_slice()[index]).into_raw()
    })
}

#[no_mangle]
pub fn skia_color_array_at_put(
    array_ptr: *mut ValueBox<BoxerArray<Color>>,
    index: usize,
    color_ptr: *mut ValueBox<Color>,
) {
    color_ptr.with_not_null_value(|color| {
        BoxerArray::<Color>::boxer_array_at_put(array_ptr, index, color)
    });
}

#[no_mangle]
pub fn skia_color_array_drop(mut ptr: *mut ValueBox<BoxerArray<Color>>) {
    ptr.drop()
}
