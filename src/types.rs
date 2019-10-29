extern crate typename;

use super::*;
use self::typename::TypeName;

#[no_mangle]
pub fn skia_scalar_name() {
    skia_safe::scalar::type_name();
}
