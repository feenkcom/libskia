use skia_safe::{Image, Matrix};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

pub struct CachedImage {
    image: Image,
    was_used: bool,
    matrix: Matrix,
}

impl CachedImage {
    pub fn new(image: Image, matrix: Matrix) -> Self {
        Self {
            image,
            was_used: false,
            matrix,
        }
    }

    pub fn mark_not_used(&mut self) {
        self.was_used = false;
    }

    pub fn mark_used(&mut self) {
        self.was_used = true;
    }
}

pub struct ImageCache {
    pub images: HashMap<u32, CachedImage>,
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

    pub fn push_id_image(&mut self, picture_id: u32, image: Image, matrix: Matrix) {
        self.images
            .insert(picture_id, CachedImage::new(image, matrix));
    }

    pub fn get_picture_image(&mut self, picture_id: u32) -> Option<(&Image, Matrix)> {
        self.images.get_mut(&picture_id).and_then(|cached_image| {
            cached_image.mark_used();
            Some((&cached_image.image, cached_image.matrix))
        })
    }

    pub fn has_cached_image(&self, picture_id: u32) -> bool {
        self.images.contains_key(&picture_id)
    }

    pub fn count_cached_images(&self) -> usize {
        self.images.len()
    }

    pub fn clear(&mut self) {
        self.images.clear();
    }

    pub fn mark_images_as_not_used(&mut self) {
        for cached_image in self.images.values_mut() {
            cached_image.mark_not_used();
        }
    }

    pub fn remove_unused_images(&mut self) {
        self.images.retain(|_, cached_image| cached_image.was_used)
    }
}
