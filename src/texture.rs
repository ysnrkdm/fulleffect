use crate::color::gamma_to_linear;
use crate::color::rgba_to_color;
use crate::math::clamp_u32;
use crate::vector::Vector3;
use image::{DynamicImage, GenericImageView};
use std::fmt;
use std::path::Path;

use crate::color::Color;
use crate::vector::Vector2;

pub struct ImageTexture {
    pub image: DynamicImage,
}

impl fmt::Debug for ImageTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Texture {{ width: {}, height: {} }}",
            self.image.width(),
            self.image.height()
        )
    }
}

impl ImageTexture {
    pub fn new(path: &str) -> ImageTexture {
        ImageTexture {
            image: image::open(&Path::new(path)).unwrap(),
        }
    }

    pub fn sample_bilinear(&self, u: f64, v: f64) -> Vector3 {
        let x = u * self.image.width() as f64;
        let y = v * self.image.height() as f64;
        let x1 = x.floor();
        let y1 = y.floor();
        let x2 = x1 + 1.0;
        let y2 = y1 + 1.0;

        let p11 = self.sample_nearest_screen(x1 as u32, y1 as u32);
        let p12 = self.sample_nearest_screen(x1 as u32, y2 as u32);
        let p21 = self.sample_nearest_screen(x2 as u32, y1 as u32);
        let p22 = self.sample_nearest_screen(x2 as u32, y2 as u32);

        let gamma = (p11 * (x2 - x) * (y2 - y)
            + p21 * (x - x1) * (y2 - y)
            + p12 * (x2 - x) * (y - y1)
            + p22 * (x - x1) * (y - y1))
            / ((x2 - x1) * (y2 - y1));
        gamma_to_linear(gamma)
    }

    fn sample_nearest_screen(&self, x: u32, y: u32) -> Vector3 {
        let x = clamp_u32(x, 0, self.image.width() - 1);
        let y = clamp_u32(self.image.height() - y, 0, self.image.height() - 1);
        rgba_to_color(self.image.get_pixel(x, y))
    }
}

#[derive(Debug)]
pub struct Texture {
    pub image_texture: Option<ImageTexture>,
    pub color: Color,
}

impl Texture {
    pub fn of_color(color: Color) -> Texture {
        Texture {
            image_texture: None,
            color,
        }
    }

    pub fn from_path(path: &str) -> Texture {
        Texture {
            image_texture: Some(ImageTexture::new(path)),
            color: Vector3::one(),
        }
    }

    pub fn white() -> Texture {
        Texture::of_color(Color::one())
    }
    pub fn black() -> Texture {
        Texture::of_color(Color::zero())
    }

    pub fn sample(&self, uv: Vector2) -> Color {
        if let Some(ref texture) = self.image_texture {
            texture.sample_bilinear(uv.x, uv.y) * self.color
        } else {
            self.color
        }
    }
}
