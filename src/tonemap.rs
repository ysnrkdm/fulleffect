use crate::color::{Color};

#[allow(dead_code)]
pub enum ToneMappingMode {
    None,
    Reinhard,
}

pub type TonemapFn = fn(color: &Color) -> Color;

pub fn none(color: &Color) -> Color { *color }
