use fulleffect::camera::{Camera, LensShape};
use fulleffect::color::Color;
use fulleffect::material::{Material, SurfaceType};
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

    let scene = Scene {
        elements: vec![
            Box::new(Sphere {
                center: Vector3::new(0.0, radius, 0.0),
                radius: radius,
                material: Material {
                    surface: SurfaceType::Diffuse,
                    albedo: Texture::white(),
                    emission: Texture::of_color(Color::all_of(0.05)),
                    roughness: Texture::of_color(Color::all_of(0.99)),
                },
            }),
            // 光源
            Box::new(Sphere {
                center: Vector3::new(3.0, 2.0 + radius, -2.0),
                radius: radius * 0.2,
                material: Material {
                    surface: SurfaceType::Diffuse,
                    albedo: Texture::black(),
                    emission: Texture::of_color(Color::new(100.0, 100.0, 200.0)),
                    roughness: Texture::of_color(Color::all_of(0.05)),
                },
            }),
        ],
        skybox: Skybox {
            px_texture: Texture::black(),
            nx_texture: Texture::black(),
            py_texture: Texture::black(),
            ny_texture: Texture::black(),
            pz_texture: Texture::black(),
            nz_texture: Texture::black(),
            intensity: Vector3::zero(),
        },
    };

    (camera, scene)
}
