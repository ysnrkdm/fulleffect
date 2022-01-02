extern crate image;

use crate::material::PointMaterial;
use image::{ImageBuffer, Rgb};
use rand::{Rng, SeedableRng, StdRng};
use rayon::prelude::*;
use std::io::stdout;
use std::io::Write;
use stopwatch::Stopwatch;

use crate::camera::{Camera, Ray};
use crate::color::{color_to_rgb, linear_to_gamma, Color};
use crate::config;
use crate::filter;
use crate::rayintersectable::Intersectable;
use crate::tonemap;
use crate::vector::{Vector2, Vector3};

use crate::scene::Illuminable;

macro_rules! b_f_1 {
    ($fn_: ident) => {
        |a| $fn_(*a)
    };
}

fn update_imgbuf(
    filter: filter::PixelArrayFilterFn,
    ldr_from_hdr: tonemap::TonemapFn,
    accumulation_buf: &Vec<Vector3>,
    sampling: u32,
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let scale = ((sampling * config::SUPER_SAMPLING * config::SUPER_SAMPLING) as f64).recip();

    let mut tmp: Vec<_> = accumulation_buf
        .par_iter()
        .map(|pixel| {
            let hdr = *pixel * scale;
            let ldr = ldr_from_hdr(&hdr);
            let gamma = linear_to_gamma(ldr);
            gamma
        })
        .collect();

    tmp = filter(tmp);

    let rgbs: Vec<_> = tmp.par_iter().map(b_f_1!(color_to_rgb)).collect();

    for (i, pixel) in imgbuf.pixels_mut().enumerate() {
        *pixel = rgbs[i];
    }
}

pub trait Renderer: Sync {
    fn max_sampling(&self) -> u32;

    fn calc_pixel(
        &self,
        scene: &dyn Illuminable,
        camera: &Camera,
        emissions: &Vec<&Box<dyn Intersectable>>,
        normalized_coord: &Vector2,
        sampling: u32,
    ) -> Color;

    fn report_progress(
        &mut self,
        accumulation_buf: &Vec<Vector3>,
        sampling: u32,
        imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> bool;

    fn filter(&self) -> filter::PixelArrayFilterFn;

    fn tonemap(&self) -> tonemap::TonemapFn;

    fn render(
        &mut self,
        scene: &dyn Illuminable,
        camera: &Camera,
        imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> u32 {
        let resolution = Vector2::new(imgbuf.width() as f64, imgbuf.height() as f64);
        let num_of_pixel = imgbuf.width() * imgbuf.height();
        let mut accumulation_buf = vec![Vector3::zero(); num_of_pixel as usize];
        let emissions = scene.emissions();

        for sampling in 1..=self.max_sampling() {
            accumulation_buf
                .par_iter_mut()
                .enumerate()
                .for_each(|(i, pixel)| {
                    let y = i as u32 / imgbuf.width();
                    let x = i as u32 - y * imgbuf.width();
                    let frag_coord = Vector2::new(x as f64, (imgbuf.height() - y) as f64);
                    *pixel += self.supersampling(
                        scene,
                        camera,
                        &emissions,
                        &frag_coord,
                        &resolution,
                        sampling,
                    );
                });

            if self.report_progress(&accumulation_buf, sampling, imgbuf) {
                return sampling;
            }
        }

        self.max_sampling()
    }

    fn supersampling(
        &self,
        scene: &dyn Illuminable,
        camera: &Camera,
        emissions: &Vec<&Box<dyn Intersectable>>,
        frag_coord: &Vector2,
        resolution: &Vector2,
        sampling: u32,
    ) -> Color {
        let mut accumulator = Color::zero();

        for sy in 0..config::SUPER_SAMPLING {
            for sx in 0..config::SUPER_SAMPLING {
                let offset =
                    Vector2::new(sx as f64, sy as f64) / config::SUPER_SAMPLING as f64 - 0.5;
                let normalized_coord =
                    ((*frag_coord + offset) * 2.0 - *resolution) / resolution.x.min(resolution.y);
                accumulator +=
                    self.calc_pixel(scene, camera, emissions, &normalized_coord, sampling);
            }
        }

        accumulator
    }

    //    fn save_progress_image(path: &str, accumulation_buf: &Vec<Vector3>, sampling: u32, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    //        // TODO
    //    }
}

pub enum DebugRenderMode {
    Shading,
    Normal,
    Depth,
    FocalPlane,
}

pub struct DebugRenderer {
    pub mode: DebugRenderMode,
    pub filter: filter::PixelArrayFilterFn,
    pub tonemap: tonemap::TonemapFn,
}

impl Renderer for DebugRenderer {
    fn max_sampling(&self) -> u32 {
        1
    }

    fn calc_pixel(
        &self,
        scene: &dyn Illuminable,
        camera: &Camera,
        _emissions: &Vec<&Box<dyn Intersectable>>,
        normalized_coord: &Vector2,
        _sampling: u32,
    ) -> Color {
        let ray = camera.ray(&normalized_coord);
        let light_direction = Vector3::new(1.0, 2.0, -1.0).normalized();
        let (hit, intersection) = scene.intersect(&ray);

        if hit {
            match self.mode {
                DebugRenderMode::Shading => {
                    let shadow_ray = Ray {
                        origin: intersection.position + intersection.normal * config::OFFSET,
                        direction: light_direction,
                    };
                    let (shadow_hit, _) = scene.intersect(&shadow_ray);
                    let shadow = if shadow_hit { 0.5 } else { 1.0 };
                    let diffuse = intersection.normal.dot(&light_direction).max(0.0);
                    intersection.material.emission + intersection.material.albedo * diffuse * shadow
                }
                DebugRenderMode::Normal => Vector3::one(),
                DebugRenderMode::Depth => Vector3::one(),
                DebugRenderMode::FocalPlane => Vector3::one(),
            }
        } else {
            intersection.material.emission
        }
    }

    fn report_progress(
        &mut self,
        accumulation_buf: &Vec<Vector3>,
        sampling: u32,
        imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> bool {
        update_imgbuf(
            self.filter(),
            self.tonemap(),
            accumulation_buf,
            sampling,
            imgbuf,
        );
        true
    }

    fn filter(&self) -> filter::PixelArrayFilterFn {
        self.filter
    }

    fn tonemap(&self) -> tonemap::TonemapFn {
        self.tonemap
    }
}

pub struct PathTracingRenderer {
    sampling: u32,
    pub filter: filter::PixelArrayFilterFn,
    pub tonemap: tonemap::TonemapFn,

    stopwatch: Stopwatch,
}

impl Renderer for PathTracingRenderer {
    fn max_sampling(&self) -> u32 {
        self.sampling
    }

    fn calc_pixel(
        &self,
        scene: &dyn Illuminable,
        camera: &Camera,
        emissions: &Vec<&Box<dyn Intersectable>>,
        normalized_coord: &Vector2,
        sampling: u32,
    ) -> Color {
        let s = ((4.0 + normalized_coord.x) * 100870.0) as usize;
        let t = ((4.0 + normalized_coord.y) * 100304.0) as usize;
        let seed: &[_] = &[8700304, sampling as usize, s, t];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut ray = camera.ray_with_dof(normalized_coord, &mut rng);

        let mut accumulation = Color::zero();
        let mut reflectance = Color::one();

        for _ in 1..config::PATHTRACING_BOUNCE_LIMIT {
            let random = rng.gen::<(f64, f64)>();
            let (hit, intersection) = scene.intersect(&ray);
            let mut current_reflectance = 1.0;

            if hit {
                let view = &-ray.direction;
                if let Some(result) = intersection.material.sample(
                    random,
                    &intersection.position,
                    view,
                    &intersection.normal,
                ) {
                    if intersection.material.nee_available() {
                        accumulation += reflectance
                            * PathTracingRenderer::next_event_estimation(
                                random,
                                &result.ray.origin,
                                view,
                                &intersection.normal,
                                scene,
                                &emissions,
                                &intersection.material,
                            );
                    }

                    ray = result.ray;
                    current_reflectance = result.reflectance;
                } else {
                    // Nothing sampled, break path tracing interations
                    break;
                }
            }

            accumulation += reflectance * intersection.material.emission;
            reflectance *= intersection.material.albedo * current_reflectance;

            if !hit || reflectance == Vector3::zero() {
                break;
            }
        }

        accumulation
    }

    fn report_progress(
        &mut self,
        accumulation_buf: &Vec<Vector3>,
        sampling: u32,
        imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> bool {
        print!(
            "rendering: {}-th sampling done. Elapsed {} ms\r",
            sampling,
            self.stopwatch.elapsed_ms()
        );
        let _ = stdout().flush();

        update_imgbuf(
            self.filter(),
            self.tonemap(),
            accumulation_buf,
            sampling,
            imgbuf,
        );
        self.stopwatch.restart();

        false
    }

    fn filter(&self) -> filter::PixelArrayFilterFn {
        self.filter
    }

    fn tonemap(&self) -> tonemap::TonemapFn {
        self.tonemap
    }
}

impl PathTracingRenderer {
    pub fn new(
        sampling: u32,
        filter: filter::PixelArrayFilterFn,
        tonemap: tonemap::TonemapFn,
    ) -> PathTracingRenderer {
        PathTracingRenderer {
            sampling,
            filter,
            tonemap,
            stopwatch: Stopwatch::new(),
        }
    }

    fn next_event_estimation(
        random: (f64, f64),
        position: &Vector3,
        view: &Vector3,
        normal: &Vector3,
        scene: &dyn Illuminable,
        emissions: &Vec<&Box<dyn Intersectable>>,
        material: &PointMaterial,
    ) -> Vector3 {
        //return Vector3::zero();

        let mut accumulation = Vector3::zero();

        for emission in emissions {
            let surface = emission.sample_on_surface(random);
            let shadow_vec = surface.position - *position;
            let shadow_dir = shadow_vec.normalized();
            let shadow_ray = Ray {
                origin: *position,
                direction: shadow_dir,
            };
            let (shadow_hit, shadow_intersection) = scene.intersect(&shadow_ray);

            if shadow_hit
                && shadow_intersection
                    .position
                    .is_approx_same_to(&surface.position)
            {
                let dot_0 = normal.dot(&shadow_dir).abs();
                let dot_l = surface.normal.dot(&shadow_dir).abs();
                let distance_pow2 = shadow_vec.dot(&shadow_vec);
                let g = (dot_0 * dot_l) / distance_pow2;
                let pdf = surface.pdf;

                accumulation += shadow_intersection.material.emission
                    * material.bsdf(view, normal, &shadow_dir)
                    * g
                    / pdf;
            }
        }

        accumulation * material.albedo
    }
}
