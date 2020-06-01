use skia_safe::{Image, Picture};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

pub struct ImageCache {
    pub images: HashMap<u32, Image>,
}

impl Debug for ImageCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("ImageCache")
            .field("images:", &self.images.keys())
            .finish()
    }
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }
    pub fn push_picture_image(&mut self, picture: &Picture, image: Image) {
        self.images.insert(picture.unique_id(), image);
    }
    pub fn push_id_image(&mut self, picture_id: u32, image: Image) {
        self.images.insert(picture_id, image);
    }

    pub fn pop_picture_image(&mut self, picture: &Picture) -> Option<Image> {
        self.images.remove(&picture.unique_id())
    }

    pub fn clear(&mut self) {
        self.images.clear();
    }
}
