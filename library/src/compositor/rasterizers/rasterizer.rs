use compositor::rasterizers::picture_rasterizer::{
    PictureRasterizer, PictureToRasterize, RasterizedPicture,
};
use compositor::rasterizers::shadow_rasterizer::{
    RasterizedShadow, ShadowRasterizer, ShadowToRasterize,
};
use skia_safe::Canvas;

pub trait Rasterizer {
    fn rasterize_picture(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<PictureToRasterize>,
    ) -> Vec<RasterizedPicture>;

    fn rasterize_shadow(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<ShadowToRasterize>,
    ) -> Vec<RasterizedShadow>;
}

pub struct SyncRasterizer {}

impl SyncRasterizer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rasterizer for SyncRasterizer {
    fn rasterize_picture(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<PictureToRasterize>,
    ) -> Vec<RasterizedPicture> {
        let mut gpu_context = canvas.recording_context();
        let mut rasterized_pictures: Vec<RasterizedPicture> = vec![];

        let picture_rasterizer = PictureRasterizer::new();

        for picture in to_rasterize {
            rasterized_pictures.push(picture_rasterizer.rasterize(picture, gpu_context.as_mut()));
        }
        rasterized_pictures
    }

    fn rasterize_shadow(
        &mut self,
        canvas: &mut Canvas,
        to_rasterize: Vec<ShadowToRasterize>,
    ) -> Vec<RasterizedShadow> {
        let mut gpu_context = canvas.recording_context();
        let mut rasterized_shadows: Vec<RasterizedShadow> = vec![];

        let shadow_rasterizer = ShadowRasterizer::new();

        for shadow in to_rasterize {
            rasterized_shadows.push(shadow_rasterizer.rasterize(shadow, gpu_context.as_mut()));
        }
        rasterized_shadows
    }
}
