extern crate image;

use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

use crate::config;
use crate::vector::{Vector3, Vector2};
use crate::camera::{Ray, Camera};
use crate::color::{Color, color_to_rgb, linear_to_gamma};
use crate::filter;
use crate::tonemap;

use crate::scene::{Illuminable};

macro_rules! b_f_1 {
    ($fn_: ident) => { |a| { $fn_(*a) } }
}

fn update_imgbuf(filter: filter::PixelArrayFilterFn, ldr_from_hdr: tonemap::TonemapFn, accumulation_buf: &Vec<Vector3>, sampling: u32, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let scale = ((sampling * config::SUPER_SAMPLING * config::SUPER_SAMPLING) as f64).recip();

    let mut tmp: Vec<_> = accumulation_buf.par_iter().map(|pixel| {
        let hdr = *pixel * scale;
        let ldr = ldr_from_hdr(&hdr);
        let gamma = linear_to_gamma(ldr);
        gamma
    }).collect();

    tmp = filter(tmp);

    let rgbs: Vec<_> = tmp.par_iter().map(b_f_1!(color_to_rgb)).collect();

    for (i, pixel) in imgbuf.pixels_mut().enumerate() {
        *pixel = rgbs[i];
    }
}

pub trait Renderer: Sync {
    //<editor-fold desc="Be overridden">
    fn max_sampling(&self) -> u32;

    fn calc_pixel(&self, scene: &Illuminable, camera: &Camera, normalized_coord: &Vector2, sampling: u32) -> Color;

    fn report_progress(&self, accumulation_buf: &Vec<Vector3>, sampling: u32, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> bool;

    fn filter(&self) -> filter::PixelArrayFilterFn;

    fn tonemap(&self) -> tonemap::TonemapFn;
    //</editor-fold>

    //<editor-fold desc="Have default impl">
    fn render(&mut self, scene: &Illuminable, camera: &Camera, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> u32 {
        let resolution = Vector2::new(imgbuf.width() as f64, imgbuf.height() as f64);
        let num_of_pixel = imgbuf.width() * imgbuf.height();
        let mut accumulation_buf = vec![Vector3::zero(); num_of_pixel as usize];

        for sampling in 1..=self.max_sampling() {
            accumulation_buf.par_iter_mut().enumerate().for_each(|(i, pixel)| {
                let y = i as u32 / imgbuf.width();
                let x = i as u32 - y * imgbuf.width();
                let frag_coord = Vector2::new(x as f64, (imgbuf.height() - y) as f64);
                *pixel += self.supersampling(scene, camera, &frag_coord, &resolution, sampling);
            });

            if self.report_progress(&accumulation_buf, sampling, imgbuf) {
                return sampling;
            }
        }

        self.max_sampling()
    }

    fn supersampling(&self, scene: &Illuminable, camera: &Camera, frag_coord: &Vector2, resolution: &Vector2, sampling: u32) -> Color {
        let mut accumulator = Color::zero();

        for sy in 0..config::SUPER_SAMPLING {
            for sx in 0..config::SUPER_SAMPLING {
                let offset = Vector2::new(sx as f64, sy as f64) / config::SUPER_SAMPLING as f64 - 0.5;
                let normalized_coord = ((*frag_coord + offset) * 2.0 - *resolution) / resolution.x.min(resolution.y);
                accumulator += self.calc_pixel(scene, camera, &normalized_coord, sampling);
            }
        }

        accumulator
    }

//    fn save_progress_image(path: &str, accumulation_buf: &Vec<Vector3>, sampling: u32, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
//        // TODO
//    }
    //</editor-fold>
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
    fn max_sampling(&self) -> u32 { 1 }

    fn calc_pixel(&self, scene: &Illuminable, camera: &Camera, normalized_coord: &Vector2, _sampling: u32) -> Color {
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
                },
                DebugRenderMode::Normal => Vector3::one(),
                DebugRenderMode::Depth => Vector3::one(),
                DebugRenderMode::FocalPlane => Vector3::one(),
            }
        } else {
            intersection.material.emission
        }
    }

    fn report_progress(&self, accumulation_buf: &Vec<Vector3>, sampling: u32, imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) -> bool {
        update_imgbuf(self.filter(), self.tonemap(), accumulation_buf, sampling, imgbuf);
        true
    }

    fn filter(&self) -> filter::PixelArrayFilterFn {
        self.filter
    }

    fn tonemap(&self) -> tonemap::TonemapFn {
        self.tonemap
    }
}