mod sample_scenes;

extern crate fulleffect;

use fulleffect::camera::Camera;
use fulleffect::filter;
use fulleffect::renderer::PathTracingRenderer;
use fulleffect::renderer::{DebugRenderMode, DebugRenderer, Renderer};
use fulleffect::scene::Scene;
use fulleffect::tonemap;
use stopwatch::Stopwatch;

fn render_and_save_image<R: Renderer>(
    renderer: &mut R,
    width: u32,
    height: u32,
    camera: &Camera,
    scene: Scene,
) -> u32 {
    let mut imgbuf = image::ImageBuffer::new(width, height);
    let sampled = renderer.render(&scene, camera, &mut imgbuf);
    let _ = image::DynamicImage::ImageRgb8(imgbuf).save("result.png");
    sampled
}

fn main() {
    println!("Start rendering...");

    let width = 640u32;
    let height = 480u32;

    // let width = 1920u32;
    // let height = 1080u32;

    let (camera, scene) = sample_scenes::simple_scene_mesh::sample_scene();

    let mut _renderer = DebugRenderer {
        filter: filter::identity_filter,
        tonemap: tonemap::none,
        mode: DebugRenderMode::Shading,
    };
    let mut path_tracing_renderer =
        PathTracingRenderer::new(10, filter::identity_filter, tonemap::none);

    let mut stopwatch = Stopwatch::start_new();
    let sampled = render_and_save_image(&mut path_tracing_renderer, width, height, &camera, scene);
    stopwatch.stop();

    println!("Rendered with {} samples", sampled);
    println!("Done rendering in {} sec", stopwatch.elapsed().as_secs());
}
