use boxer::boxes::{ReferenceBox, ReferenceBoxPointer};
use boxer::{ValueBox, ValueBoxPointer};
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use skia_safe::{Canvas, Image, Matrix, Picture, Vector};
use std::collections::VecDeque;

use compositor::rasterizers::picture_rasterizer::PictureToRasterize;
use compositor::rasterizers::rasterizer::{AsyncRasterizer, Rasterizer, SyncRasterizer};
use compositor::rasterizers::shadow_rasterizer::ShadowToRasterize;
use compositor::shadow_cache::{Shadow, ShadowCache};
use glutin::event_loop::EventLoop;
use glutin::{PossiblyCurrent, WindowedContext};
use std::cell::RefCell;
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct MatrixContext {
    pub matrix: Matrix,
}

impl MatrixContext {
    pub fn new() -> Self {
        Self {
            matrix: Matrix::new_identity(),
        }
    }
    pub fn new_matrix(matrix: Matrix) -> Self {
        Self { matrix }
    }
    pub fn concat(&self, matrix: &Matrix) -> Self {
        Self::new_matrix(Matrix::concat(&self.matrix, &matrix))
    }
    pub fn translate(&self, vector: Vector) -> Self {
        Self::new_matrix(Matrix::concat(&self.matrix, &Matrix::translate(vector)))
    }
}

pub struct CompositorContext<'canvas, 'compositor> {
    pub image_cache: &'compositor mut ImageCache,
    pub shadow_cache: &'compositor mut ShadowCache,
    pub pictures_to_rasterize: Vec<PictureToRasterize>,
    pub shadows_to_rasterize: Vec<ShadowToRasterize>,
    pub canvas: &'canvas mut Canvas,
    pub transformations: VecDeque<Matrix>,
}

impl<'canvas, 'compositor> CompositorContext<'canvas, 'compositor> {
    pub fn new(
        canvas: &'canvas mut Canvas,
        image_cache: &'compositor mut ImageCache,
        shadow_cache: &'compositor mut ShadowCache,
    ) -> Self {
        let mut transformations = VecDeque::new();
        transformations.push_back(canvas.total_matrix());

        Self {
            image_cache,
            shadow_cache,
            pictures_to_rasterize: Vec::new(),
            shadows_to_rasterize: Vec::new(),
            canvas,
            transformations,
        }
    }

    pub fn push_matrix(&mut self, matrix: Matrix) -> &mut Self {
        self.transformations.push_back(matrix);
        self
    }

    pub fn push_translate(&mut self, vector: Vector) -> &mut Self {
        self.push_matrix(Matrix::concat(
            self.current_matrix(),
            &Matrix::translate(vector),
        ));
        self
    }

    pub fn push_concat(&mut self, matrix: &Matrix) -> &mut Self {
        self.push_matrix(Matrix::concat(self.current_matrix(), matrix));
        self
    }

    pub fn pop_matrix(&mut self) -> &mut Self {
        self.transformations.pop_back();
        self
    }

    pub fn current_matrix(&self) -> &Matrix {
        self.transformations.back().unwrap()
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        self.canvas
    }

    pub fn mark_caches_not_used(&mut self) {
        self.image_cache.mark_images_as_not_used();
        self.shadow_cache.mark_images_as_not_used();
    }

    pub fn purge_caches(&mut self) {
        self.image_cache.remove_unused_images();
        self.shadow_cache.remove_unused_images();
    }
}

/// Image cache related implementation
impl<'canvas, 'compositor> CompositorContext<'canvas, 'compositor> {
    pub fn add_picture_to_rasterize(&mut self, picture: Arc<Picture>) {
        if self.is_picture_rasterized(picture.unique_id()) {
            return;
        }

        self.pictures_to_rasterize.push(PictureToRasterize::new(
            picture,
            self.current_matrix().clone(),
        ));
    }

    pub fn sort_pictures_to_rasterize(&mut self) {
        self.pictures_to_rasterize.sort_by(|a, b| {
            let left_area = a.bounds.size().height * a.bounds.size().width;
            let right_area = b.bounds.size().height * b.bounds.size().width;
            right_area.partial_cmp(&left_area).unwrap()
        });
    }

    pub fn get_pictures_to_rasterize(&self, amount: usize) -> Vec<PictureToRasterize> {
        let mut pictures = vec![];
        for picture in &self.pictures_to_rasterize {
            if pictures.len() >= amount {
                break;
            } else {
                pictures.push(picture.clone());
            }
        }
        pictures
    }

    pub fn add_rasterized_picture_image(&mut self, picture_id: u32, image: Image, matrix: Matrix) {
        self.image_cache.push_id_image(picture_id, image, matrix);
    }

    pub fn is_picture_rasterized(&self, picture_id: u32) -> bool {
        self.image_cache.has_cached_image(picture_id)
    }

    pub fn get_rasterized_picture_image_and_canvas(
        &mut self,
        picture_id: u32,
    ) -> (Option<(&Image, Matrix)>, &mut Canvas) {
        let image = self.image_cache.get_picture_image(picture_id);
        (image, self.canvas)
    }
}

/// Shadow cache related implementation
impl<'canvas, 'compositor> CompositorContext<'canvas, 'compositor> {
    pub fn add_shadow_to_rasterize(&mut self, shadow: Shadow) {
        if self.is_shadow_rasterized(&shadow) {
            return;
        }

        self.shadows_to_rasterize.push(ShadowToRasterize::new(
            shadow,
            self.current_matrix().clone(),
        ));
    }

    pub fn is_shadow_rasterized(&self, shadow: &Shadow) -> bool {
        self.shadow_cache.has_cached_shadow(shadow)
    }

    pub fn get_shadows_to_rasterize(&self) -> Vec<ShadowToRasterize> {
        let mut shadows = vec![];
        for shadow in &self.shadows_to_rasterize {
            shadows.push(shadow.clone())
        }
        shadows
    }

    pub fn add_rasterized_shadow_image(&mut self, shadow: Shadow, image: Image, matrix: Matrix) {
        self.shadow_cache.push_shadow_image(shadow, image, matrix);
    }

    pub fn get_rasterized_shadow_image_and_canvas(
        &mut self,
        shadow: &Shadow,
    ) -> (Option<(&Image, Matrix)>, &mut Canvas) {
        let image = self.shadow_cache.get_shadow_image(shadow);
        (image, self.canvas)
    }
}

pub struct Compositor {
    pub rasterizer: Box<dyn Rasterizer>,
    pub image_cache: ImageCache,
    pub images_per_frame: usize,
    pub shadow_cache: ShadowCache,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            image_cache: ImageCache::new(),
            rasterizer: Box::new(SyncRasterizer::new()),
            images_per_frame: 5,
            shadow_cache: ShadowCache::new(),
        }
    }

    pub fn new_software(workers_num: usize) -> Self {
        Self {
            image_cache: ImageCache::new(),
            rasterizer: Box::new(AsyncRasterizer::new(workers_num)),
            images_per_frame: 5,
            shadow_cache: ShadowCache::new(),
        }
    }

    pub fn new_accelerated(
        workers_num: usize,
        event_loop: &EventLoop<()>,
        windowed_context: &WindowedContext<PossiblyCurrent>,
    ) -> Self {
        Self {
            image_cache: ImageCache::new(),
            rasterizer: Box::new(AsyncRasterizer::new_accelerated(
                workers_num,
                event_loop,
                windowed_context,
            )),
            images_per_frame: 5,
            shadow_cache: ShadowCache::new(),
        }
    }

    pub fn set_images_limit(&mut self, images_per_frame: usize) {
        self.images_per_frame = images_per_frame;
    }

    pub fn draw(&mut self, layers_tree: &Rc<RefCell<dyn Layer>>, canvas: &mut Canvas) {
        let mut context =
            CompositorContext::new(canvas, &mut self.image_cache, &mut self.shadow_cache);

        context.mark_caches_not_used();

        layers_tree.borrow_mut().prepare(&mut context);

        context.sort_pictures_to_rasterize();

        for rasterized_picture in self.rasterizer.rasterize_picture(
            context.canvas,
            context.get_pictures_to_rasterize(self.images_per_frame),
        ) {
            let picture_id = rasterized_picture.picture_id();
            let image = rasterized_picture.image;
            let matrix = rasterized_picture.matrix;
            match image {
                None => {}
                Some(image) => {
                    context.add_rasterized_picture_image(picture_id, image, matrix);
                }
            }
        }

        for rasterized_shadow in self
            .rasterizer
            .rasterize_shadow(context.canvas, context.get_shadows_to_rasterize())
        {
            let shadow = rasterized_shadow.shadow;
            let image = rasterized_shadow.image;
            let matrix = rasterized_shadow.matrix;
            match image {
                None => {}
                Some(image) => context.add_rasterized_shadow_image(shadow, image, matrix),
            }
        }

        layers_tree.borrow_mut().draw(&mut context);

        context.purge_caches();
    }
}

#[no_mangle]
pub fn skia_compositor_new() -> *mut ValueBox<Compositor> {
    ValueBox::new(Compositor::new()).into_raw()
}

#[no_mangle]
pub fn skia_compositor_new_accelerated(
    workers_num: usize,
    _event_loop_ptr: *mut ValueBox<EventLoop<()>>,
    _windowed_context_ptr: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) -> *mut ValueBox<Compositor> {
    _event_loop_ptr.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        _windowed_context_ptr.with_not_null_return(std::ptr::null_mut(), |context| {
            ValueBox::new(Compositor::new_accelerated(
                workers_num,
                event_loop,
                context,
            ))
            .into_raw()
        })
    })
}

#[no_mangle]
pub fn skia_compositor_new_software(workers_num: usize) -> *mut ValueBox<Compositor> {
    ValueBox::new(Compositor::new_software(workers_num)).into_raw()
}

#[no_mangle]
pub fn skia_compositor_draw(
    compositor_ptr: *mut ValueBox<Compositor>,
    layers_tree_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
    canvas_ptr: *mut ReferenceBox<Canvas>,
) -> *mut c_void {
    canvas_ptr.with_not_null_return(std::ptr::null_mut(), |canvas| {
        layers_tree_ptr.with_not_null_value_return(std::ptr::null_mut(), |layers_tree| {
            compositor_ptr.with_not_null_return(std::ptr::null_mut(), |compositor| {
                compositor.draw(&layers_tree, canvas);
                std::ptr::null_mut()
            })
        })
    })
}

#[no_mangle]
pub fn skia_compositor_drop(mut ptr: *mut ValueBox<Compositor>) {
    ptr.drop();
}
