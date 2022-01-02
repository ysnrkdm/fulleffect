use rand::{Rng, StdRng};

use crate::vector::{Vector2, Vector3};

#[derive(Debug)]
pub enum LensShape {
    Square,
    Circle,
}

#[derive(Debug)]
pub struct Camera {
    pub position: Vector3, // Camera position in the world

    pub lens_shape: LensShape,

    pub lens_radius: f64,
    pub focus_distance: f64,

    pub right: Vector3,   // basis vector for right
    pub up: Vector3,      // same for up
    pub forward: Vector3, // same for forward

    pub plane_half_right: Vector3,
    pub plane_half_up: Vector3,
}

#[derive(Clone, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Camera {
    pub fn new(
        position: Vector3,
        target: Vector3,
        y_up: Vector3,
        v_fov: f64,
        lens_shape: LensShape,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let lens_radius = 0.5 * aperture;
        let plane_half_height = v_fov.to_radians().tan();
        let forward = (target - position).normalized();
        let right = forward.cross(&y_up).normalized();
        let up = right.cross(&forward).normalized();

        Camera {
            position,
            lens_shape,
            lens_radius,
            focus_distance,
            forward,
            right,
            up,
            plane_half_right: right * plane_half_height * focus_distance,
            plane_half_up: up * plane_half_height * focus_distance,
        }
    }

    fn compose_ray(&self, normalized_coord: &Vector2, camera_position_offset: Vector3) -> Ray {
        Ray {
            origin: self.position + camera_position_offset,
            direction: (normalized_coord.x * self.plane_half_right
                + normalized_coord.y * self.plane_half_up
                + self.focus_distance * self.forward)
                .normalized(),
        }
    }

    pub fn ray(&self, normalized_coord: &Vector2) -> Ray {
        self.compose_ray(normalized_coord, Vector3::new(0.0, 0.0, 0.0))
    }

    fn sample_on_lens(&self, rng: &mut StdRng) -> Vector2 {
        loop {
            let (u, v) = rng.gen::<(f64, f64)>();
            let square = Vector2::new(2.0 * u - 1.0, 2.0 * v - 1.0);

            match self.lens_shape {
                LensShape::Square => {
                    return square;
                }
                LensShape::Circle => {
                    if square.norm() < 1.0 {
                        return square;
                    }
                }
            }
        }
    }

    pub fn ray_with_dof(&self, normalized_coord: &Vector2, rng: &mut StdRng) -> Ray {
        let lens_uv = self.sample_on_lens(rng) * self.lens_radius;
        let lens_pos = self.right * lens_uv.x + self.up * lens_uv.y;
        self.compose_ray(normalized_coord, lens_pos)
    }
}
