extern crate typename;

use super::*;
use self::typename::TypeName;

#[no_mangle]
pub fn skia_scalar_name() {
    CBox:: skia_safe::scalar::type_name());
}

#[test]
pub fn test_scalar_name(){
    assert_eq!(skia_scalar_name(), "f32");
}