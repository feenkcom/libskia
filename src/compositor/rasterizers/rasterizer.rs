use compositor::rasterizers::picture_rasterizer::{
    PictureRasterizer, PictureToRasterize, RasterizedPicture,
};
use compositor::rasterizers::rasterizer_thread::{
    RasterizerRequest, RasterizerResult, RasterizerThread,
};
use glutin::event_loop::EventLoop;
use glutin::{ContextBuilder, PossiblyCurrent, WindowedContext};
use skia_safe::{Canvas, IRect, Image, Matrix, Picture, Rect, RoundOut};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::JoinHandle;

pub trait Rasterizer {
    fn rasterize(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<PictureToRasterize>,
    ) -> Vec<RasterizedPicture>;
}

pub struct AsyncRasterizer {
    pub sender: Sender<RasterizerRequest>,
    pub receiver: Receiver<RasterizerResult>,
    pub thread: Option<JoinHandle<()>>,
}

impl AsyncRasterizer {
    pub fn new(workers_num: usize) -> Self {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();

        let thread = std::thread::spawn(move || {
            let thread = RasterizerThread::new(out_tx, in_rx);
            thread.run(workers_num, None);
        });

        Self {
            sender: in_tx,
            receiver: out_rx,
            thread: Some(thread),
        }
    }

    pub fn new_accelerated(
        workers_num: usize,
        event_loop: &EventLoop<()>,
        windowed_context: &WindowedContext<PossiblyCurrent>,
    ) -> Self {
        let contexts = (0..workers_num)
            .map(|i| {
                ContextBuilder::new()
                    .with_shared_lists(windowed_context)
                    .build_headless(event_loop, glutin::dpi::PhysicalSize::new(1, 1))
            })
            .filter(|option| option.is_ok())
            .map(|option| option.unwrap())
            .collect();

        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();

        let thread = std::thread::spawn(move || {
            let thread = RasterizerThread::new(out_tx, in_rx);
            thread.run(workers_num, Some(contexts));
        });

        Self {
            sender: in_tx,
            receiver: out_rx,
            thread: Some(thread),
        }
    }
}

impl Rasterizer for AsyncRasterizer {
    fn rasterize(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<PictureToRasterize>,
    ) -> Vec<RasterizedPicture> {
        let mut rasterized_pictures: Vec<RasterizedPicture> = vec![];

        self.sender.send(RasterizerRequest::Rasterize(to_rasterize));
        match self.receiver.recv() {
            Ok(RasterizerResult::Rasterized(images)) => {
                rasterized_pictures = images;
            }
            Err(_) => {}
        }
        rasterized_pictures
    }
}

impl Drop for AsyncRasterizer {
    fn drop(&mut self) {
        self.sender.send(RasterizerRequest::Terminate).unwrap();

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

pub struct SyncRasterizer {}

impl SyncRasterizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rasterizer for SyncRasterizer {
    fn rasterize(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<PictureToRasterize>,
    ) -> Vec<RasterizedPicture> {
        let mut gpu_context = canvas.gpu_context();
        let mut rasterized_pictures: Vec<RasterizedPicture> = vec![];

        let picture_rasterizer = PictureRasterizer::new();

        for mut picture in to_rasterize {
            rasterized_pictures.push(picture_rasterizer.rasterize(picture, gpu_context.as_mut()));
        }
        rasterized_pictures
    }
}
