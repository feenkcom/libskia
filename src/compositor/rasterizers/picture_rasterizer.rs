use boxer::function;
use compositor::compositor_stats::{
    RasterizationStats, RasterizationStepStats, RasterizerSurfaceType,
};
use skia_safe::gpu::{Context, SurfaceOrigin};
use skia_safe::{
    Budgeted, Color, ColorSpace, IRect, Image, ImageInfo, Matrix, Picture, Rect, RoundOut, Surface,
    Vector,
};
use std::fmt::{Debug, Error, Formatter};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// I contain all the necessary data to rasterize a picture
#[derive(Clone)]
pub struct PictureToRasterize {
    pub picture: Arc<Picture>,
    pub bounds: Rect,
    pub matrix: Matrix,
}

impl PictureToRasterize {
    pub fn new(picture: Arc<Picture>, matrix: Matrix) -> Self {
        let logical_bounds = picture.cull_rect();
        Self {
            picture,
            bounds: logical_bounds,
            matrix,
        }
    }

    pub fn device_bounds(&self) -> IRect {
        Self::compute_device_bounds(&self.bounds, &self.matrix)
    }

    pub fn into_rasterized(
        self,
        image: Option<Image>,
        stats: RasterizationStats,
    ) -> RasterizedPicture {
        RasterizedPicture::new(self.picture, image, self.matrix, stats)
    }

    pub fn compute_device_bounds(bounds: &Rect, matrix: &Matrix) -> IRect {
        match matrix.map_rect_scale_translate(bounds) {
            None => bounds.round_out(),
            Some(bounds) => bounds.round_out(),
        }
    }

    pub fn compute_device_bounds_rect(bounds: &Rect, matrix: &Matrix) -> Rect {
        match matrix.map_rect_scale_translate(bounds) {
            None => bounds.clone(),
            Some(bounds) => bounds,
        }
    }
}

impl Debug for PictureToRasterize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("PictureToRasterize")
            .field("id", &self.picture.unique_id())
            .field("bounds", &self.bounds)
            .finish()
    }
}

/// I hold a result of the picture rasterization. The image is [`Some`] if the process
/// was successful
pub struct RasterizedPicture {
    pub picture: Arc<Picture>,
    pub image: Option<Image>,
    pub matrix: Matrix,
    pub stats: RasterizationStats,
}

impl RasterizedPicture {
    pub fn new(picture: Arc<Picture>, image: Option<Image>, matrix: Matrix, stats: RasterizationStats) -> Self {
        Self {
            picture,
            image,
            matrix,
            stats,
        }
    }

    pub fn picture_id(&self) -> u32 {
        self.picture.unique_id()
    }
}

/// I convert a Picture [`PictureToRasterize`] into an Image
pub struct PictureRasterizer {}
impl PictureRasterizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rasterize(
        &self,
        mut picture_to_rasterize: PictureToRasterize,
        gpu_context: Option<&mut Context>,
    ) -> RasterizedPicture {
        let device_bounds = picture_to_rasterize.device_bounds();
        let picture = &picture_to_rasterize.picture;
        let picture_id = picture.unique_id();

        let mut stats = RasterizationStats::new(picture_id);
        let start_time = std::time::Instant::now();

        let image_info = ImageInfo::new_n32_premul(device_bounds.size(), ColorSpace::new_srgb());

        let surface = match gpu_context {
            None => None,
            Some(mut context) => {
                let gpu_surface_time = std::time::Instant::now();
                match Surface::new_render_target(
                    context,
                    Budgeted::Yes,
                    &image_info,
                    0,
                    SurfaceOrigin::BottomLeft,
                    None,
                    false,
                ) {
                    Some(surface) => {
                        stats.log(gpu_surface_time, String::from("Create GPU Surface"));
                        stats.set_surface_type(RasterizerSurfaceType::GPU);
                        Some(surface)
                    }
                    None => {
                        if cfg!(debug_assertions) {
                            eprintln!(
                                "[{}] Could not create GPU surface of size {:?}",
                                function!(),
                                image_info.dimensions()
                            );
                        }
                        None
                    }
                }
            }
        };

        let surface = match surface {
            None => {
                let cpu_surface_time = std::time::Instant::now();
                match Surface::new_raster(&image_info, None, None) {
                    None => {
                        if cfg!(debug_assertions) {
                            eprintln!(
                                "[{}] Could not create CPU surface of size {:?}",
                                function!(),
                                image_info.dimensions()
                            );
                        }
                        None
                    }
                    Some(mut surface) => {
                        stats.log(cpu_surface_time, String::from("Create CPU Surface"));
                        stats.set_surface_type(RasterizerSurfaceType::Software);
                        Some(surface)
                    }
                }
            }
            Some(surface) => Some(surface),
        };

        let image = match surface {
            None => None,
            Some(mut surface) => {
                let draw_picture_time = std::time::Instant::now();

                let canvas = surface.canvas();
                canvas.clear(Color::TRANSPARENT);
                canvas.translate(Vector::new(
                    -device_bounds.left as f32,
                    -device_bounds.top as f32,
                ));
                canvas.concat(&picture_to_rasterize.matrix);

                canvas.draw_picture(&picture, None, None);

                stats.log(draw_picture_time, String::from("Draw picture"));

                let canvas_flush = std::time::Instant::now();
                stats.log(canvas_flush, String::from("Flush canvas"));

                let raster_image_snapshot = std::time::Instant::now();
                let mut image = Some(surface.image_snapshot());
                stats.log(raster_image_snapshot, String::from("Image snapshot"));
                image
            }
        };

        stats.log_total(start_time);

        picture_to_rasterize.into_rasterized(image, stats)
    }
}
