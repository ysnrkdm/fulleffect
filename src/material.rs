use crate::color::{Color};
use crate::camera::{Ray};
use crate::texture::{Texture};

#[derive(Clone, Debug)]
pub enum SurfaceType {
    Diffuse,
}

pub struct SampleResult {
    pub ray: Ray,
}

#[derive(Debug)]
pub struct Material {
    pub surface: SurfaceType,
    pub albedo: Texture,
    pub emission: Texture,
    pub roughness: Texture,
}

#[derive(Clone, Debug)]
pub struct PointMaterial {
    pub surface: SurfaceType,
    pub albedo: Color,
    pub emission: Color,
    pub roughness: f64,
}