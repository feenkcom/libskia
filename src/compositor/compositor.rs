use boxer::boxes::{ReferenceBox, ReferenceBoxPointer, ValueBox, ValueBoxPointer};
use compositor::image_cache::ImageCache;
use compositor::layers::layer::Layer;
use skia_safe::{
    scalar, Budgeted, Canvas, Color, Color4f, ColorSpace, Image, ImageInfo, Matrix, Paint, Picture,
    Rect, Surface, Vector,
};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

use compositor::rasterizers::picture_rasterizer::{PictureToRasterize, RasterizedPicture};
use compositor::rasterizers::rasterizer::{AsyncRasterizer, Rasterizer, SyncRasterizer};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::{ContextBuilder, PossiblyCurrent, WindowedContext};
use skia_safe::gpu::SurfaceOrigin;
use std::cell::RefCell;
use std::os::raw::c_void;
use std::rc::Rc;
use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub struct RasterizerContext {
    pub matrix: Matrix,
}

impl RasterizerContext {
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
        Self::new_matrix(Matrix::concat(&self.matrix, &Matrix::new_trans(vector)))
    }
}

pub struct Compositor {
    pub rasterizer: Box<dyn Rasterizer>,
    pub image_cache: ImageCache,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            image_cache: ImageCache::new(),
            rasterizer: Box::new(SyncRasterizer::new()),
        }
    }

    pub fn new_software(workers_num: usize) -> Self {
        Self {
            image_cache: ImageCache::new(),
            rasterizer: Box::new(AsyncRasterizer::new(workers_num)),
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
        }
    }

    pub fn draw(&mut self, mut layers_tree: &Rc<RefCell<dyn Layer>>, canvas: &mut Canvas) {
        let start_time = Instant::now();

        layers_tree
            .borrow_mut()
            .take_image_from_cache(&mut self.image_cache);

        let mut to_rasterize = vec![];
        layers_tree.borrow_mut().take_picture_to_rasterize(
            RasterizerContext::new_matrix(canvas.total_matrix()),
            &mut to_rasterize,
        );
        to_rasterize.sort_by(|a, b| {
            let left_area = a.bounds.size().height * a.bounds.size().width;
            let right_area = b.bounds.size().height * b.bounds.size().width;
            right_area.partial_cmp(&left_area).unwrap()
        });

        let mut rasterized_pictures = HashMap::new();
        for rasterized_picture in self.rasterizer.rasterize(canvas, to_rasterize) {
            let picture = rasterized_picture.picture;
            let picture_id = picture.unique_id();
            let image = rasterized_picture.image;
            rasterized_pictures.insert(picture_id, picture);
            match image {
                None => {}
                Some(image) => {
                    self.image_cache.push_id_image(picture_id, image);
                }
            }
        }

        layers_tree
            .borrow_mut()
            .put_picture_after_rasterization(&mut rasterized_pictures);
        layers_tree
            .borrow_mut()
            .take_image_from_cache(&mut self.image_cache);

        self.image_cache.clear();

        layers_tree
            .borrow_mut()
            .draw_on(RasterizerContext::new_matrix(canvas.total_matrix()), canvas);

        layers_tree
            .borrow_mut()
            .put_image_in_cache(&mut self.image_cache);
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
    _compositor_ptr: *mut ValueBox<Compositor>,
    _layers_tree_ptr: *mut ValueBox<Rc<RefCell<dyn Layer>>>,
    _canvas_ptr: *mut ReferenceBox<Canvas>,
) -> *mut c_void {
    _canvas_ptr.with_not_null_return(std::ptr::null_mut(), |canvas| {
        _layers_tree_ptr.with_not_null_value_return_block(
            || std::ptr::null_mut(),
            |layers_tree| {
                _compositor_ptr.with_not_null_return(std::ptr::null_mut(), |compositor| {
                    compositor.draw(&layers_tree, canvas);
                    std::ptr::null_mut()
                })
            },
        )
    })
}

#[no_mangle]
pub fn skia_compositor_drop(_ptr: *mut ValueBox<Compositor>) {
    _ptr.drop();
}
