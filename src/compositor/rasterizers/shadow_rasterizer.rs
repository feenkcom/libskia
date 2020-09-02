use boxer::function;
use compositor::compositor_stats::{RasterizationStats, RasterizerSurfaceType};
use compositor::shadow_cache::Shadow;
use skia_safe::gpu::{Context, SurfaceOrigin};
use skia_safe::image_filters::drop_shadow_only;
use skia_safe::paint::Style;
use skia_safe::{
    Budgeted, Color, ColorSpace, IRect, Image, ImageInfo, Matrix, Paint, Rect, RoundOut, Surface,
    Vector,
};

#[derive(Debug, Clone)]
pub struct ShadowToRasterize {
    pub shadow: Shadow,
    pub matrix: Matrix,
    pub bounds: Rect,
}

impl ShadowToRasterize {
    pub fn new(shadow: Shadow, matrix: Matrix) -> Self {
        let logical_bounds = shadow.cull_rect();
        Self {
            shadow,
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
    ) -> RasterizedShadow {
        RasterizedShadow::new(self.shadow, self.matrix, image, stats)
    }

    pub fn compute_device_bounds(bounds: &Rect, matrix: &Matrix) -> IRect {
        matrix.map_rect(bounds).0.round_out()
    }
}

/// I hold a result of the shadow rasterization. The image is [`Some`] if the process
/// was successful
pub struct RasterizedShadow {
    pub shadow: Shadow,
    pub image: Option<Image>,
    pub matrix: Matrix,
    pub stats: RasterizationStats,
}

impl RasterizedShadow {
    pub fn new(
        shadow: Shadow,
        matrix: Matrix,
        image: Option<Image>,
        stats: RasterizationStats,
    ) -> Self {
        Self {
            shadow,
            image,
            matrix,
            stats,
        }
    }
}

/// I convert a Shadow [`ShadowToRasterize`] into an Image
pub struct ShadowRasterizer {}
impl ShadowRasterizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rasterize(
        &self,
        shadow_to_rasterize: ShadowToRasterize,
        gpu_context: Option<&mut Context>,
    ) -> RasterizedShadow {
        let device_bounds = shadow_to_rasterize.device_bounds();
        let shadow = &shadow_to_rasterize.shadow;

        let mut stats = RasterizationStats::new(0);
        let start_time = std::time::Instant::now();

        let image_info = ImageInfo::new_n32_premul(device_bounds.size(), ColorSpace::new_srgb());

        let surface = match gpu_context {
            None => None,
            Some(context) => {
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
                    Some(surface) => {
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
                let draw_shadow_time = std::time::Instant::now();

                let canvas = surface.canvas();
                canvas.clear(Color::TRANSPARENT);
                canvas.translate(Vector::new(
                    -device_bounds.left as f32,
                    -device_bounds.top as f32,
                ));
                canvas.concat(&shadow_to_rasterize.matrix);

                let drop_shadow_filter =
                    drop_shadow_only(shadow.offset, shadow.radius, shadow.color, None, None);

                let mut shadow_paint = Paint::default();
                shadow_paint.set_style(Style::Stroke);
                shadow_paint.set_color(Color::WHITE);
                shadow_paint.set_stroke_width(if shadow.radius.0 > shadow.radius.1 {
                    shadow.radius.0
                } else {
                    shadow.radius.1
                });
                shadow_paint.set_image_filter(drop_shadow_filter);

                canvas.draw_path(&shadow.path, &shadow_paint);

                stats.log(draw_shadow_time, String::from("Draw shadow"));

                let canvas_flush = std::time::Instant::now();
                stats.log(canvas_flush, String::from("Flush canvas"));

                let raster_image_snapshot = std::time::Instant::now();
                let image = Some(surface.image_snapshot());
                stats.log(raster_image_snapshot, String::from("Image snapshot"));
                image
            }
        };

        stats.log_total(start_time);

        shadow_to_rasterize.into_rasterized(image, stats)
    }
}
