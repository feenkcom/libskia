use skia_safe::{Image, Picture, Color, scalar, Path};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq, Hash)]
pub struct Shadow {
    pub color: Color,
    pub width: scalar,
    pub path: Path,
}

use std::hash::{Hash, Hasher};

impl Hash for Shadow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.into_native().hash(state);
        self.width.has
    }
}

impl Debug for Shadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Shadow")
            .field("color:", &self.color)
            .field("width:", &self.width)
            .finish()
    }
}

pub struct ShadowCache {
    pub images: HashMap<Shadow, Image>,
}

impl Debug for ShadowCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("ShadowCache")
            .field("images:", &self.images.keys())
            .finish()
    }
}

impl ShadowCache {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.images.clear();
    }
}
