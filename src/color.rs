use boxer::array::BoxerArray;
use boxer::boxes::{ValueBox, ValueBoxPointer};
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
pub fn skia_color_get_red(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.r())
}

#[no_mangle]
pub fn skia_color_get_green(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.g())
}

#[no_mangle]
pub fn skia_color_get_blue(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.b())
}

#[no_mangle]
pub fn skia_color_get_alpha(_ptr: *mut ValueBox<Color>) -> u8 {
    _ptr.with(|color| color.a())
}

#[no_mangle]
pub fn skia_color_drop(_ptr: *mut ValueBox<Color>) {
    _ptr.drop();
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
    element_ptr.with_value(|color| BoxerArray::<Color>::boxer_array_create_with(color, amount))
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
    _array_ptr: *mut ValueBox<BoxerArray<Color>>,
    index: usize,
) -> *mut ValueBox<Color> {
    _array_ptr.with(|array| ValueBox::new(array.to_slice()[index]).into_raw())
}

#[no_mangle]
pub fn skia_color_array_at_put(
    _array_ptr: *mut ValueBox<BoxerArray<Color>>,
    index: usize,
    _color_ptr: *mut ValueBox<Color>,
) {
    _color_ptr
        .with_value(|color| BoxerArray::<Color>::boxer_array_at_put(_array_ptr, index, color));
}

#[no_mangle]
pub fn skia_color_array_drop(_ptr: *mut ValueBox<BoxerArray<Color>>) {
    _ptr.drop()
}
