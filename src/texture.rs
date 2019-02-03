use image::{DynamicImage, GenericImageView};
use std::fmt;

use crate::vector::Vector2;
use crate::color::Color;

pub struct ImageTexture {
    pub image: DynamicImage,
}

impl fmt::Debug for ImageTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Texture {{ width: {}, height: {} }}", self.image.width(), self.image.height())
    }
}

#[derive(Debug)]
pub struct Texture {
//    pub image_texture: Option<ImageTexture>,
    pub color: Color,
}

impl Texture {
    pub fn of_color(color: Color) -> Texture {
        Texture { color }
    }

    pub fn white() -> Texture { Texture::of_color(Color::one()) }
    pub fn black() -> Texture { Texture::of_color(Color::zero()) }

    pub fn sample(&self, uv: Vector2) -> Color {
        self.color
    }
}