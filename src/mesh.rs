use crate::aabb::Aabb;
use crate::camera::Ray;
use crate::config::INF;
use crate::material::Material;
use crate::math::det;
use crate::rayintersectable::Intersectable;
use crate::rayintersectable::Intersection;
use crate::scene::Surface;
use crate::vector::Vector2;
use crate::vector::Vector3;

trait IntersectWith<T> {
    fn intersect_with(&self, _: &T, _: &Ray, _: &mut Intersection) -> bool;
}

pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
}

pub struct Face {
    pub v0: usize,
    pub v1: usize,
    pub v2: usize,
}

pub struct Mesh {
    pub vertexes: Vec<Vector3>,
    pub faces: Vec<Face>,
    pub material: Material,
}

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray, intersection: &mut Intersection) -> bool {
        for face in &self.faces {
            if triangle_intesected_with_ray(
                &Triangle {
                    v0: self.vertexes[face.v0],
                    v1: self.vertexes[face.v1],
                    v2: self.vertexes[face.v2],
                },
                ray,
                intersection,
            ) {
                return true;
            }
        }
        false
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

pub struct BvhNode {
    pub aabb: Aabb,

    // size must be 0 or 2
    // empty means leaf node
    pub children: Vec<Box<BvhNode>>,

    // has faces means leaf node
    pub indexes: Vec<usize>,
}

fn node_from_mesh_with_indexes(mesh: &Mesh, face_indexes: &mut Vec<usize>) -> BvhNode {
    let mut node = BvhNode::empty();
    node.set_aabb_from_mesh(mesh, face_indexes);

    let mid = face_indexes.len() / 2;
    if mid <= 2 {
        // set leaf node
        node.indexes = face_indexes.clone();
    } else {
        // set intermediate node
        let lx = node.aabb.max.x - node.aabb.min.x;
        let ly = node.aabb.max.y - node.aabb.min.y;
        let lz = node.aabb.max.z - node.aabb.min.z;

        if lx > ly && lx > lz {
            face_indexes.sort_by(|a, b| {
                let a_face = &mesh.faces[*a];
                let b_face = &mesh.faces[*b];
                let a_sum = mesh.vertexes[a_face.v0].x
                    + mesh.vertexes[a_face.v1].x
                    + mesh.vertexes[a_face.v2].x;
                let b_sum = mesh.vertexes[b_face.v0].x
                    + mesh.vertexes[b_face.v1].x
                    + mesh.vertexes[b_face.v2].x;
                a_sum.partial_cmp(&b_sum).unwrap()
            });
        } else if ly > lx && ly > lz {
            face_indexes.sort_by(|a, b| {
                let a_face = &mesh.faces[*a];
                let b_face = &mesh.faces[*b];
                let a_sum = mesh.vertexes[a_face.v0].y
                    + mesh.vertexes[a_face.v1].y
                    + mesh.vertexes[a_face.v2].y;
                let b_sum = mesh.vertexes[b_face.v0].y
                    + mesh.vertexes[b_face.v1].y
                    + mesh.vertexes[b_face.v2].y;
                a_sum.partial_cmp(&b_sum).unwrap()
            });
        } else {
            face_indexes.sort_by(|a, b| {
                let a_face = &mesh.faces[*a];
                let b_face = &mesh.faces[*b];
                let a_sum = mesh.vertexes[a_face.v0].z
                    + mesh.vertexes[a_face.v1].z
                    + mesh.vertexes[a_face.v2].z;
                let b_sum = mesh.vertexes[b_face.v0].z
                    + mesh.vertexes[b_face.v1].z
                    + mesh.vertexes[b_face.v2].z;
                a_sum.partial_cmp(&b_sum).unwrap()
            });
        }

        let mut left_face_indexes = face_indexes.split_off(mid);
        node.children
            .push(Box::new(node_from_mesh_with_indexes(mesh, face_indexes)));
        node.children.push(Box::new(node_from_mesh_with_indexes(
            mesh,
            &mut left_face_indexes,
        )));
    }

    node
}

impl From<&Mesh> for BvhNode {
    fn from(mesh: &Mesh) -> Self {
        let mut face_indexes: Vec<usize> = (0..mesh.faces.len()).collect();
        node_from_mesh_with_indexes(mesh, &mut face_indexes)
    }
}

impl IntersectWith<Mesh> for BvhNode {
    fn intersect_with(&self, mesh: &Mesh, ray: &Ray, intersection: &mut Intersection) -> bool {
        if !self.aabb.intersect_with_ray(ray).0 {
            return false;
        }

        let mut any_hit = false;
        if self.children.is_empty() {
            // leaf node
            for face_index in &self.indexes {
                let face = &mesh.faces[*face_index];
                if triangle_intesected_with_ray(
                    &Triangle {
                        v0: mesh.vertexes[face.v0],
                        v1: mesh.vertexes[face.v1],
                        v2: mesh.vertexes[face.v2],
                    },
                    ray,
                    intersection,
                ) {
                    any_hit = true;
                }
            }
        } else {
            // intermediate node
            for child in &self.children {
                if child.intersect_with(mesh, ray, intersection) {
                    any_hit = true;
                }
            }
        }

        any_hit
    }
}

impl BvhNode {
    pub fn empty() -> BvhNode {
        BvhNode {
            aabb: Aabb {
                min: Vector3::new(INF, INF, INF),
                max: Vector3::new(-INF, -INF, -INF),
            },
            children: vec![],
            indexes: vec![],
        }
    }

    fn set_aabb_from_mesh(&mut self, mesh: &Mesh, face_indexes: &Vec<usize>) {
        for face_index in face_indexes {
            let face = &mesh.faces[*face_index];
            let v0 = &mesh.vertexes[face.v0];
            let v1 = &mesh.vertexes[face.v1];
            let v2 = &mesh.vertexes[face.v2];
            self.aabb = self.aabb.merged(&Aabb::from(Triangle {
                v0: *v0,
                v1: *v1,
                v2: *v2,
            }));
        }
    }
}

pub struct BvhMesh {
    pub mesh: Mesh,
    pub bvh: BvhNode,
}

impl From<Mesh> for BvhMesh {
    fn from(mesh: Mesh) -> Self {
        let bvh = BvhNode::from(&mesh);
        BvhMesh { mesh, bvh }
    }
}

impl Intersectable for BvhMesh {
    fn intersect(&self, ray: &Ray, intersection: &mut Intersection) -> bool {
        self.bvh.intersect_with(&self.mesh, ray, intersection)
    }
    fn material(&self) -> &Material {
        &self.mesh.material
    }
    fn nee_available(&self) -> bool {
        false
    }
    fn sample_on_surface(&self, _: (f64, f64)) -> Surface {
        unimplemented!()
    }
}

fn triangle_intesected_with_ray(
    triangle: &Triangle,
    ray: &Ray,
    intersection: &mut Intersection,
) -> bool {
    let Triangle { v0, v1, v2 } = triangle;
    let ray_inv = -ray.direction;
    let edge1 = *v1 - *v0;
    let edge2 = *v2 - *v0;
    let denominator = det(&edge1, &edge2, &ray_inv);
    if denominator == 0.0 {
        return false;
    }

    let denominator_inv = denominator.recip();
    let d = ray.origin - *v0;

    let u = det(&d, &edge2, &ray_inv) * denominator_inv;
    if u < 0.0 || u > 1.0 {
        return false;
    }

    let v = det(&edge1, &d, &ray_inv) * denominator_inv;
    if v < 0.0 || u + v > 1.0 {
        return false;
    };

    let t = det(&edge1, &edge2, &d) * denominator_inv;
    if t < 0.0 || t > intersection.distance {
        return false;
    }

    intersection.position = ray.origin + ray.direction * t;
    intersection.normal = edge1.cross(&edge2).normalized();
    intersection.distance = t;
    intersection.uv = Vector2::new(u, v);
    true
}
