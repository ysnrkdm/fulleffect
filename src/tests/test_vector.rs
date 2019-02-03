#[cfg(test)]
//#[macro_use]

//extern crate quickcheck;

use super::super::vector;

#[test]
fn test_norm() {
    let a = vector::Vector3::zero();
    assert_eq!(a.norm(), 0.0);

    let a = vector::Vector3::one();
    assert_eq!(a.norm(), 3.0);

    let a = vector::Vector3::new(1.0, 0.0, 0.0);
    assert_eq!(a.norm(), 1.0);

    let a = vector::Vector3::new(0.0, 1.0, 0.0);
    assert_eq!(a.norm(), 1.0);

    let a = vector::Vector3::new(0.0, 0.0, 1.0);
    assert_eq!(a.norm(), 1.0);
}

#[test]
fn test_length() {
    let a = vector::Vector3::zero();
    assert_eq!(a.length(), 0.0);

    let a = vector::Vector3::one();
    assert_eq!(a.length(), (3.0 as f64).sqrt());

    let a = vector::Vector3::new(1.0, 0.0, 0.0);
    assert_eq!(a.length(), 1.0);

    let a = vector::Vector3::new(0.0, 1.0, 0.0);
    assert_eq!(a.length(), 1.0);

    let a = vector::Vector3::new(0.0, 0.0, 1.0);
    assert_eq!(a.length(), 1.0);
}

#[test]
fn test_normalized() {

}

#[test]
fn test_dot() {

}

#[test]
fn test_cross() {

}

#[test]
fn test_reflect() {

}

#[test]
fn test_refract() {

}