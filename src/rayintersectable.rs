use crate::config;
use crate::vector::{Vector2, Vector3};
use crate::color::{Color};
use crate::material::{PointMaterial, Material, SurfaceType};
use crate::camera::{Ray};

#[derive(Debug)]
pub struct Intersection {
    pub position: Vector3,
    pub distance: f64,
    pub normal: Vector3,
    pub uv: Vector2,
    pub material: PointMaterial,
}

impl Intersection {
    pub fn empty() -> Intersection {
        Intersection {
            position: Vector3::zero(),
            distance: config::INF,
            normal: Vector3::zero(),
            uv: Vector2::zero(),
            material: PointMaterial {
                surface: SurfaceType::Diffuse,
                albedo: Color::one(),
                emission: Color::zero(),
                roughness: 0.2,
            },
        }
    }
}

pub trait Intersectable: Sync {
    fn intersect(&self, ray: &Ray, intersection: &mut Intersection) -> bool;
    fn material(&self) -> &Material;
}

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, intersection: &mut Intersection) -> bool {
        let a: Vector3 = ray.origin - self.center;
        let b = a.dot(&ray.direction);
        let c = a.dot(&a) - self.radius * self.radius;
        let d = b * b - c;
        let t = -b - d.sqrt();
        if d > 0.0 && 0.0 < t && t < intersection.distance {
            intersection.position = ray.origin + ray.direction * t;
            intersection.distance = t;
            intersection.normal = (intersection.position - self.center).normalized();

            intersection.uv.y = 1.0 - intersection.normal.y.acos() / config::PI;
            intersection.uv.x = 0.5
                - intersection.normal.z.signum()
                * (intersection.normal.x / intersection.normal.xz().length()).acos()
                / config::PI2;
            true
        } else {
            false
        }
    }
    fn material(&self) -> &Material { &self.material }
}