use std::f64;

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;
pub const PI2: f64 = 2.0 * PI;

pub const EPS: f64 = 1e-4;
pub const OFFSET: f64 = 1e-4;

pub const SUPER_SAMPLING: u32 = 2;

pub const GAMMA_FACTOR: f64 = 2.2;

// Tone Mapping
pub const TONE_MAPPING_EXPOSURE: f64 = 1.5;
pub const TONE_MAPPING_WHITE_POINT: f64 = 20.0;

// Denoising - Bilateral Fileter
pub const BILATERAL_FILTER_ITERATION: u32 = 1;
pub const BILATERAL_FILTER_DIAMETER: u32 = 3;
pub const BILATERAL_FILTER_SIGMA_I: f64 = 1.0;
pub const BILATERAL_FILTER_SIGMA_S: f64 = 16.0;

//
pub const PATHTRACING_BOUNCE_LIMIT: u32 = 10;
