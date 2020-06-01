use compositor::rasterizers::picture_rasterizer::{
    PictureRasterizer, PictureToRasterize, RasterizedPicture,
};
use compositor::thread_pool::{GpuContext, ThreadPool};
use glutin::{Context, NotCurrent};
use skia_safe::gpu::SurfaceOrigin;
use skia_safe::image::CachingHint;
use skia_safe::{
    Budgeted, Color, Color4f, ColorSpace, IRect, ISize, Image, ImageInfo, Matrix, Paint, Picture,
    Pixmap, Rect, RoundOut, Surface, Vector,
};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

pub enum RasterizerRequest {
    Rasterize(Vec<PictureToRasterize>),
    Terminate,
}

pub enum RasterizerResult {
    Rasterized(Vec<RasterizedPicture>),
}

pub struct RasterizerThread {
    receiver: Receiver<RasterizerRequest>,
    sender: Sender<RasterizerResult>,
}

impl RasterizerThread {
    pub fn new(sender: Sender<RasterizerResult>, receiver: Receiver<RasterizerRequest>) -> Self {
        Self { receiver, sender }
    }

    pub fn run(&self, workers_num: usize, contexts: Option<Vec<Context<NotCurrent>>>) {
        let mut pool = ThreadPool::new(workers_num, contexts);
        loop {
            match self.receiver.recv() {
                Ok(RasterizerRequest::Rasterize(mut pictures)) => {
                    let n_pics = pictures.len();
                    let (tx, rx) = channel();
                    for mut picture in pictures {
                        let tx = tx.clone();
                        pool.execute(move |gpu_context| {
                            tx.send(PictureRasterizer::new().rasterize(picture, gpu_context));
                        });
                    }
                    let result = rx.iter().take(n_pics).collect();
                    self.send(RasterizerResult::Rasterized(result))
                }
                Ok(RasterizerRequest::Terminate) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
    }

    /// Send a message to the main thread
    pub fn send(&self, msg: RasterizerResult) {
        self.sender.send(msg).unwrap();
    }
}
