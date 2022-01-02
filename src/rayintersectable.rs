use crate::aabb::Aabb;
use crate::camera::Ray;
use crate::color::Color;
use crate::config;
use crate::material::{Material, PointMaterial, SurfaceType};
use crate::math::equals_eps;
use crate::scene::Surface;
use crate::vector::{Vector2, Vector3};

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

    fn nee_available(&self) -> bool;
    fn sample_on_surface(&self, random: (f64, f64)) -> Surface;
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

    fn material(&self) -> &Material {
        &self.material
    }

    fn nee_available(&self) -> bool {
        true
    }

    // http://apollon.issp.u-tokyo.ac.jp/~watanabe/pdf/prob.pdf
    fn sample_on_surface(&self, random: (f64, f64)) -> Surface {
        let theta = config::PI2 * random.0;
        let unit_z = 1.0 - 2.0 * random.1;
        let a = (1.0 - unit_z * unit_z).sqrt();

        let normal = Vector3::new(a * theta.cos(), a * theta.sin(), unit_z);
        let position = self.center + (self.radius + config::OFFSET) * normal;
        let pdf = (4.0 * config::PI * self.radius * self.radius).recip();
        Surface {
            position,
            normal,
            pdf,
        }
    }
}

pub struct Cuboid {
    pub aabb: Aabb,
    pub material: Material,
}

impl Intersectable for Cuboid {
    fn intersect(&self, ray: &Ray, intersection: &mut Intersection) -> bool {
        let (hit, distance) = self.aabb.intersect_with_ray(ray);
        if hit && distance < intersection.distance {
            intersection.position = ray.origin + ray.direction * distance;
            intersection.distance = distance;
            let uvw = (intersection.position - self.aabb.min) / (self.aabb.max - self.aabb.min);

            if equals_eps(intersection.position.y, self.aabb.max.y) {
                intersection.normal = Vector3::new(0.0, 1.0, 0.0);
                intersection.uv = uvw.xiz();
            } else if equals_eps(intersection.position.y, self.aabb.min.y) {
                intersection.normal = Vector3::new(0.0, -1.0, 0.0);
                intersection.uv = uvw.xiz();
            } else if equals_eps(intersection.position.x, self.aabb.min.x) {
                intersection.normal = Vector3::new(-1.0, 0.0, 0.0);
                intersection.uv = uvw.zy();
            } else if equals_eps(intersection.position.x, self.aabb.max.x) {
                intersection.normal = Vector3::new(1.0, 0.0, 0.0);
                intersection.uv = uvw.zy();
            } else if equals_eps(intersection.position.z, self.aabb.min.z) {
                intersection.normal = Vector3::new(0.0, 0.0, -1.0);
                intersection.uv = uvw.xy();
            } else if equals_eps(intersection.position.z, self.aabb.max.z) {
                intersection.normal = Vector3::new(0.0, 0.0, 1.0);
                intersection.uv = uvw.xy();
            }

            true
        } else {
            false
        }
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn nee_available(&self) -> bool {
        false
    }

    fn sample_on_surface(&self, _: (f64, f64)) -> Surface {
        unimplemented!()
    }
}
