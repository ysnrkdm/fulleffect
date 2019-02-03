use crate::color::{Color};

pub enum FilterMode {
    Identity,
}

pub type PixelArrayFilterFn = fn(Vec<Color>) -> Vec<Color>;

pub fn identity_filter(pixel_array: Vec<Color>) -> Vec<Color> {
    pixel_array.clone()
}
