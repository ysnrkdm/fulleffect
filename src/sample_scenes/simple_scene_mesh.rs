use fulleffect::aabb::Aabb;
use fulleffect::camera::{Camera, LensShape};
use fulleffect::color::Color;
use fulleffect::loader::ObjLoader;
use fulleffect::material::{Material, SurfaceType};
use fulleffect::matrix::Matrix44;
use fulleffect::mesh::BvhMesh;
use fulleffect::rayintersectable::Cuboid;
use fulleffect::rayintersectable::Sphere;
use fulleffect::scene::{Scene, Skybox};
use fulleffect::texture::Texture;
use fulleffect::vector::Vector3;

pub fn sample_scene() -> (Camera, Scene) {
    let camera = Camera::new(
        Vector3::new(0.0, 2.0, 9.0),              // eye
        Vector3::new(0.0, 1.0, 0.0),              // target
        Vector3::new(0.0, 1.0, 0.0).normalized(), // y_up
        10.0,                                     // fov
        LensShape::Circle,                        // lens shape
        0.2 * 0.0,                                // aperture
        8.8,                                      // focus_distance
    );

    let radius = 0.6;
    let light_intensity_coef = 4000.0;

    let scene = Scene {
        elements: vec![
            Box::new(Sphere {
                center: Vector3::new(0.0, radius, 0.0),
                radius: radius,
                material: Material {
                    surface: SurfaceType::Diffuse,
                    albedo: Texture::white(),
                    emission: Texture::black(),
                    roughness: Texture::of_color(Color::all_of(0.99)),
                },
            }),
            // Rabbit L
            Box::new(BvhMesh::from(ObjLoader::load(
                "resources/models/bunny/bunny_face1000_flip.obj",
                Matrix44::scale_linear(1.5)
                    * Matrix44::translate(-1.2, 0.0, 0.0)
                    * Matrix44::rotate_y(-0.2),
                Material {
                    surface: SurfaceType::GGX { f0: 0.8 },
                    albedo: Texture::of_color(Color::new(1.0, 0.04, 0.04)),
                    emission: Texture::black(),
                    roughness: Texture::of_color(Color::all_of(0.1)),
                },
            ))),
            // Rabbit R
            Box::new(BvhMesh::from(ObjLoader::load(
                "resources/models/bunny/bunny_face1000.obj",
                Matrix44::scale_linear(1.5)
                    * Matrix44::translate(1.2, 0.0, 0.0)
                    * Matrix44::rotate_y(0.2),
                Material {
                    surface: SurfaceType::Refraction {
                        refractive_index: 1.5,
                    },
                    albedo: Texture::of_color(Color::new(0.7, 0.7, 1.0)),
                    emission: Texture::black(),
                    roughness: Texture::of_color(Color::all_of(0.1)),
                },
            ))),
            // Light
            Box::new(Sphere {
                center: Vector3::new(3.0, 3.0 + radius, -2.0),
                radius: radius * 0.2,
                material: Material {
                    surface: SurfaceType::Diffuse,
                    albedo: Texture::black(),
                    emission: Texture::of_color(Color::new(
                        1.0 * light_intensity_coef,
                        0.2 * light_intensity_coef,
                        0.2 * light_intensity_coef,
                    )),
                    roughness: Texture::of_color(Color::all_of(0.05)),
                },
            }),
            // Light
            Box::new(Sphere {
                center: Vector3::new(-3.0, 3.0 + radius, -2.0),
                radius: radius * 0.2,
                material: Material {
                    surface: SurfaceType::Diffuse,
                    albedo: Texture::black(),
                    emission: Texture::of_color(Color::new(
                        0.2 * light_intensity_coef,
                        1.0 * light_intensity_coef,
                        0.2 * light_intensity_coef,
                    )),
                    roughness: Texture::of_color(Color::all_of(0.05)),
                },
            }),
            // Floor
            Box::new(Cuboid {
                aabb: Aabb {
                    min: Vector3::new(-5.0, -1.0, -5.0),
                    max: Vector3::new(5.0, 0.0, 5.0),
                },
                material: Material {
                    surface: SurfaceType::GGX { f0: 0.8 },
                    albedo: Texture::from_path(
                        "resources/textures/2d/checkered_diagonal_10_0.5_1.0_512.png",
                    ),
                    emission: Texture::black(),
                    roughness: Texture::from_path(
                        "resources/textures/2d/checkered_diagonal_10_0.1_0.6_512.png",
                    ),
                },
            }),
        ],
        skybox: Skybox {
            px_texture: Texture::white(),
            nx_texture: Texture::white(),
            py_texture: Texture::white(),
            ny_texture: Texture::white(),
            pz_texture: Texture::white(),
            nz_texture: Texture::white(),
            intensity: Vector3::all_of(0.5),
        },
    };

    (camera, scene)
}
