//extern crate image;

use image::Rgb;
use image::Rgba;

use crate::config;
use crate::math::saturate;
use crate::vector::Vector3;
//use crate::config;

pub type Color = Vector3;

pub fn color_to_rgb(color: Color) -> Rgb<u8> {
    Rgb([
        (255.0 * saturate(color.x)) as u8,
        (255.0 * saturate(color.y)) as u8,
        (255.0 * saturate(color.z)) as u8,
    ])
}

pub fn rgba_to_color(color: Rgba<u8>) -> Color {
    Color::new(
        color.0[0] as f64 / 255.0,
        color.0[1] as f64 / 255.0,
        color.0[2] as f64 / 255.0,
    )
}

fn gamma_to_linear_f64(v: f64) -> f64 {
    v.powf(config::GAMMA_FACTOR)
}

pub fn gamma_to_linear(color: Color) -> Color {
    Color::new(
        gamma_to_linear_f64(color.x),
        gamma_to_linear_f64(color.y),
        gamma_to_linear_f64(color.z),
    )
}

fn linear_to_gamma_f64(v: f64) -> f64 {
    v.powf(config::GAMMA_FACTOR.recip())
}

pub fn linear_to_gamma(color: Color) -> Color {
    Color::new(
        linear_to_gamma_f64(color.x),
        linear_to_gamma_f64(color.y),
        linear_to_gamma_f64(color.z),
    )
}

pub fn color_to_luminance(color: &Color) -> f64 {
    Color::new(0.22, 0.707, 0.071).dot(color)
}
