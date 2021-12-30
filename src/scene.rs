use crate::vector::{Vector3};
use crate::color::{Color};
use crate::texture::{Texture};
use crate::camera::{Ray};
use crate::rayintersectable::{Intersection, Intersectable};

pub struct Surface {
    pub position: Vector3,
    pub normal: Vector3,
    pub pdf: f64,
}

pub trait Illuminable: Sync {
    fn intersect(&self, ray: &Ray) -> (bool, Intersection);
}

pub struct Scene {
    pub elements: Vec<Box<dyn Intersectable>>,
    pub skybox: Skybox,
}

impl Illuminable for Scene {
    fn intersect(&self, ray: &Ray) -> (bool, Intersection) {
        let mut intersection = Intersection::empty();
        let mut nearest: Option<&Box<dyn Intersectable>> = None;

        for e in &self.elements {
            if e.intersect(&ray, &mut intersection) {
                nearest = Some(&e);
            }
        }

        if let Some(element) = nearest {
            let material = element.material();
            intersection.material.surface = material.surface.clone();
            intersection.material.albedo = material.albedo.sample(intersection.uv);
            intersection.material.emission = material.emission.sample(intersection.uv);
            intersection.material.roughness = material.roughness.sample(intersection.uv).x;
            (true, intersection)
        } else {
            intersection.material.emission = self.skybox.sample(&ray.direction);
            (false, intersection)
        }
    }
}

pub struct Skybox {
    pub px_texture: Texture,
    pub nx_texture: Texture,
    pub py_texture: Texture,
    pub ny_texture: Texture,
    pub pz_texture: Texture,
    pub nz_texture: Texture,
    pub intensity: Vector3,
}

impl Skybox {
    pub fn sample(&self, _direction: &Vector3) -> Color {
        self.intensity * self.px_texture.color
    }
}
