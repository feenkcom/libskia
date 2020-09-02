use ordered_float::OrderedFloat;
use skia_safe::{scalar, Color, Image, Matrix, Path, Rect, Vector};
use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::hash::{Hash, Hasher};

pub struct CachedShadowImage {
    image: Image,
    frames_to_purge: usize,
    matrix: Matrix,
}

impl CachedShadowImage {
    pub fn new(image: Image, matrix: Matrix) -> Self {
        Self {
            image,
            frames_to_purge: 1000,
            matrix,
        }
    }

    pub fn mark_not_used(&mut self) {
        self.frames_to_purge = max(self.frames_to_purge - 1, 0);
    }

    pub fn mark_used(&mut self) {
        self.frames_to_purge = min(self.frames_to_purge + 1, 1000);
    }

    pub fn should_purge(&self) -> bool {
        self.frames_to_purge <= 0
    }
}

#[derive(PartialEq, Clone)]
pub struct Shadow {
    pub color: Color,
    pub radius: (scalar, scalar),
    pub offset: Vector,
    pub path: Path,
    pub hash: Option<u64>,
}

impl Shadow {
    pub fn new(color: Color, radius: (scalar, scalar), offset: Vector, path: Path) -> Self {
        let mut shadow = Self {
            color,
            radius,
            offset,
            path,
            hash: None,
        };
        shadow.hash = Some(shadow.compute_default_hash());
        shadow
    }

    pub fn compute_default_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.compute_hash(&mut hasher);
        hasher.finish()
    }

    pub fn compute_hash<H: Hasher>(&self, state: &mut H) {
        self.color.r().hash(state);
        self.color.g().hash(state);
        self.color.b().hash(state);
        self.color.a().hash(state);
        OrderedFloat::from(self.offset.x).hash(state);
        OrderedFloat::from(self.offset.y).hash(state);
        OrderedFloat::from(self.radius.0).hash(state);
        OrderedFloat::from(self.radius.1).hash(state);

        let mut points = Vec::with_capacity(self.path.count_points());
        self.path.get_points(&mut points);

        for point in points {
            OrderedFloat::from(point.x).hash(state);
            OrderedFloat::from(point.y).hash(state);
        }

        let mut verbs = Vec::with_capacity(self.path.count_verbs());
        self.path.get_verbs(&mut verbs);
        verbs.hash(state);

        self.path.fill_type().hash(state);
    }

    pub fn cull_rect(&self) -> Rect {
        let bounds = self.path.bounds();
        bounds
            .with_outset(Vector::new(self.radius.0 * 3.0, self.radius.1 * 3.0))
            .with_offset(self.offset)
    }
}

impl Hash for Shadow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.hash {
            None => state.write_u64(self.compute_default_hash()),
            Some(hash) => state.write_u64(hash),
        }
    }
}

impl Eq for Shadow {}

impl Debug for Shadow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Shadow")
            .field("color:", &self.color)
            .field("radius:", &self.radius)
            .field("offset:", &self.offset)
            .finish()
    }
}

pub struct ShadowCache {
    pub images: HashMap<Shadow, CachedShadowImage>,
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

    pub fn get_shadow_image(&mut self, shadow: &Shadow) -> Option<(&Image, Matrix)> {
        self.images.get_mut(shadow).and_then(|cached_image| {
            cached_image.mark_used();
            Some((&cached_image.image, cached_image.matrix))
        })
    }

    pub fn has_cached_shadow(&self, shadow: &Shadow) -> bool {
        self.images.contains_key(shadow)
    }

    pub fn push_shadow_image(&mut self, shadow: Shadow, image: Image, matrix: Matrix) {
        self.images
            .insert(shadow, CachedShadowImage::new(image, matrix));
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
        self.images
            .retain(|_, cached_image| !cached_image.should_purge())
    }
}
