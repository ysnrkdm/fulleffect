use crate::camera::Ray;
use crate::vector::Vector3;

pub struct Aabb {
    pub min: Vector3,
    pub max: Vector3,
}

impl Aabb {
    pub fn intersect_with_aabb(&self, other: &Aabb) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn intersect_with_ray(&self, ray: &Ray) -> (bool, f64) {
        let dir_inv = Vector3::new(
            ray.direction.x.recip(),
            ray.direction.y.recip(),
            ray.direction.z.recip(),
        );

        let t1 = (self.min.x - ray.origin.x) * dir_inv.x;
        let t2 = (self.max.x - ray.origin.x) * dir_inv.x;
        let t3 = (self.min.y - ray.origin.y) * dir_inv.y;
        let t4 = (self.max.y - ray.origin.y) * dir_inv.y;
        let t5 = (self.min.z - ray.origin.z) * dir_inv.z;
        let t6 = (self.max.z - ray.origin.z) * dir_inv.z;
        let tmin = (t1.min(t2).max(t3.min(t4))).max(t5.min(t6));
        let tmax = (t1.max(t2).min(t3.max(t4))).min(t5.max(t6));

        let hit = tmin <= tmax && tmax.is_sign_positive();
        let distance = if tmin.is_sign_positive() { tmin } else { tmax };
        (hit, distance)
    }

    pub fn merged(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: Vector3::new(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.y),
                self.min.z.min(other.min.z),
            ),
            max: Vector3::new(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.y),
                self.max.z.max(other.max.z),
            ),
        }
    }
}
