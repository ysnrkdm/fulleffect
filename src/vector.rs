//#![feature(trace_macros)]

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, MulAssign};
use std::cmp::{PartialEq};

use crate::math::{saturate};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    //<editor-fold desc="Constructors">
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {x, y, z}
    }

    pub fn all_of(v: f64) -> Vector3 {
        Vector3 {x: v, y: v, z: v}
    }

    pub fn zero() -> Vector3 {
        Vector3::all_of(0.0)
    }

    pub fn one() -> Vector3 {
        Vector3::all_of(1.0)
    }

    //</editor-fold>

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalized(&self) -> Vector3 {
        let inv_len = self.length().recip();
        Vector3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn saturated(&self) -> Vector3 {
        Vector3{ x: saturate(self.x), y: saturate(self.y), z: saturate(self.z) }
    }

    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        (*self) - 2.0 * self.dot(&normal) * (*normal)
    }

    pub fn refract(&self, normal: &Vector3, refractive_index: f64) -> Vector3 {
        let k = 1.0 - refractive_index * refractive_index * (1.0 - normal.dot(self) * self.dot(normal));
        if k < 0.0 {
            Vector3::zero()
        } else {
            refractive_index * *self - (refractive_index * self.dot(normal) + k.sqrt()) * *normal
        }
    }

    pub fn xy(&self) -> Vector2 {
        Vector2::new(self.x, self.y)
    }

    pub fn zy(&self) -> Vector2 {
        Vector2::new(self.z, self.y)
    }

    pub fn xz(&self) -> Vector2 {
        Vector2::new(self.x, self.z)
    }
}

macro_rules! impl_op_v2v_for {
    ($trait_: ident, $templ_type_: ident, $for_: ident, $op_: ident, $($member_: ident),*) => {
        impl $trait_<$templ_type_> for $for_  {
            type Output = $for_;

            fn $op_(self, other: $for_) -> $for_ {
                $for_ {
                    $($member_: $trait_::$op_(self.$member_, other.$member_),)*
                }
            }
        }
    }
}

macro_rules! impl_op_v2f_for {
    ($trait_: ident, $templ_type_: ident, $for_: ident, $op_: ident, $($member_: ident),*) => {
        impl $trait_<$templ_type_> for $for_  {
            type Output = $for_;

            fn $op_(self, other: $templ_type_) -> $for_ {
                $for_ {
                    $(
                        $member_: $trait_::$op_(self.$member_, other),
                    )*
                }
            }
        }
    }
}

macro_rules! impl_ops_for_xyz {
    ($macro_name_: ident, $to_type_: ident, $with_type: ident, $(($trait_: ident, $op_fn_: ident)),*) => {
        $(
            $macro_name_!($trait_, $with_type, $to_type_, $op_fn_, x, y, z);
        )*
    }
}

//trace_macros!(true);

impl_ops_for_xyz!(impl_op_v2v_for, Vector3, Vector3, (Add, add), (Sub, sub), (Mul, mul), (Div, div));
impl_ops_for_xyz!(impl_op_v2f_for, Vector3, f64, (Add, add), (Sub, sub), (Mul, mul), (Div, div));

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        other * self
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 {x, y}
    }

    pub fn all_of(v: f64) -> Vector2 {
        Vector2 {x: v, y: v}
    }

    pub fn zero() -> Vector2 {
        Vector2::all_of(0.0)
    }

    pub fn norm(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalized(&self) -> Vector2 {
        let inv_len = self.length().recip();
        Vector2 {
            x: self.x * inv_len,
            y: self.y * inv_len,
        }
    }

    pub fn dot(&self, other: Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Vector2) -> f64 {
        self.x * other.y - other.x * self.y
    }
}

macro_rules! impl_ops_for_xy {
    ($macro_name_: ident, $to_type_: ident, $with_type: ident, $(($trait_: ident, $op_fn_: ident)),*) => {
        $(
            $macro_name_!($trait_, $with_type, $to_type_, $op_fn_, x, y);
        )*
    }
}

//trace_macros!(true);

impl_ops_for_xy!(impl_op_v2v_for, Vector2, Vector2, (Add, add), (Sub, sub), (Mul, mul), (Div, div));
impl_ops_for_xy!(impl_op_v2f_for, Vector2, f64, (Add, add), (Sub, sub), (Mul, mul), (Div, div));

impl Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, other: Vector2) -> Vector2 {
        other * self
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2 {
            x: -self.x,
            y: -self.y
        }
    }
}
