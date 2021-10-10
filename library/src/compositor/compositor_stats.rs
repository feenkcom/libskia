use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct RasterizationStepStats {
    pub duration: Duration,
    pub step_name: String,
}

impl RasterizationStepStats {
    pub fn new(duration: Duration, step_name: String) -> Self {
        Self {
            duration,
            step_name,
        }
    }
}

#[derive(Debug)]
pub enum RasterizerSurfaceType {
    Software,
    GPU,
    Unknown,
}

#[derive(Debug)]
pub struct RasterizationStats {
    pub id: u32,
    pub surface_type: RasterizerSurfaceType,
    pub step_stats: Vec<RasterizationStepStats>,
    pub total_duration: Duration,
}

impl RasterizationStats {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            surface_type: RasterizerSurfaceType::Unknown,
            step_stats: vec![],
            total_duration: Duration::default(),
        }
    }

    pub fn set_surface_type(&mut self, surface_type: RasterizerSurfaceType) {
        self.surface_type = surface_type;
    }

    pub fn log(&mut self, start_time: Instant, step: String) {
        self.step_stats.push(RasterizationStepStats::new(
            std::time::Instant::now() - start_time,
            step,
        ));
    }

    pub fn log_total(&mut self, start_time: Instant) {
        self.total_duration = std::time::Instant::now() - start_time;
    }
}
