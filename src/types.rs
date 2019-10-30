extern crate typename;

use super::*;
use self::typename::TypeName;
use skia_safe::ISize;
use boxer::size::{BoxerSizeI32};



#[no_mangle]
pub fn skia_scalar_name() {
    skia_safe::scalar::type_name();
}
