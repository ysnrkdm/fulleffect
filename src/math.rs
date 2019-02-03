use crate::config;
use crate::vector::Vector3;

pub fn modulo(a: f64, b: f64) -> f64 {
    let ret = a % b;
    if ret.is_sign_positive() { ret } else { ret + b }
}

pub fn clamp(v: f64, min: f64, max: f64) -> f64 {
    v.max(min).min(max)
}

pub fn saturate(v: f64) -> f64 {
    clamp(v, 0.0, 1.0)
}

pub fn equals_eps(a: f64, b: f64) -> bool {
    (a - b).abs() < config::EPS
}

pub fn det(a: &Vector3, b: &Vector3, c: &Vector3) -> f64 {
    (a.x * b.y * c.z)
        + (a.y * b.z * c.x)
        + (a.z * b.x * c.y)
        - (a.x * b.z * c.y)
        - (a.y * b.x * c.z)
        - (a.z * b.y * c.x)
}

pub fn mix(x: &Vector3, y: &Vector3, a: f64) -> Vector3 {
    *x * (1.0 - a) + *y * a
}
