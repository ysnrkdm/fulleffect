use crate::vector::Vector3;
use std::cmp::PartialEq;
use std::ops::{Index, IndexMut, Mul};

// matrix[nth_row][mth_col]
#[derive(Clone, Debug, Copy)]
pub struct Matrix44 {
    pub elements: [[f64; 4]; 4],
}

macro_rules! swap {
    ($a_: expr, $b_: expr) => {
        let tmp = $a_;
        $a_ = $b_;
        $b_ = tmp;
    };
}

impl Matrix44 {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn all_of(v : f64) -> Matrix44 {
        Matrix44 {
            elements: [[v, v, v, v],
                [v, v, v, v],
                [v, v, v, v],
                [v, v, v, v]]
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn identity() -> Matrix44 {
        Matrix44 {
            elements: [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]]
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn diag_of(x: f64, y: f64, z: f64, w: f64) -> Matrix44 {
        Matrix44 {
            elements: [[x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, w]]
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn scale(x: f64, y: f64, z: f64) -> Matrix44 {
        Matrix44::diag_of(x, y, z, 1.0)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn scale_linear(scale_factor: f64) -> Matrix44 {
        Matrix44::scale(scale_factor, scale_factor, scale_factor)
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotate_x(t : f64) -> Matrix44 {
        let sin = t.sin();
        let cos = t.cos();
        Matrix44 {
            elements: [
                [1.0, 0.0,  0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin,  cos, 0.0],
                [0.0, 0.0,  0.0, 1.0]]
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotate_y(t : f64) -> Matrix44 {
        let sin = t.sin();
        let cos = t.cos();
        Matrix44 {
            elements: [
                [ cos, 0.0, sin, 0.0],
                [ 0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [ 0.0, 0.0, 0.0, 1.0]]
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotate_z(t : f64) -> Matrix44 {
        let sin = t.sin();
        let cos = t.cos();
        Matrix44 {
            elements: [
                [cos, -sin, 0.0, 0.0],
                [sin,  cos, 0.0, 0.0],
                [0.0,  0.0, 1.0, 0.0],
                [0.0,  0.0, 0.0, 1.0]]
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn translate(x: f64, y: f64, z: f64) -> Matrix44 {
        Matrix44 {
            elements: [
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0]]
        }
    }

    pub fn inverse(&self) -> Matrix44 {
        let mut s = Matrix44::identity();
        let mut t = self.clone();
        // Forward elimination
        for i in 0..3 {
            // Find the maximum pivot row for i_th column
            let mut pivot = i;
            let mut pivotsize = t[i][i].abs();
            for j in (i + 1)..4 {
                let tmp = t[j][i].abs();
                if tmp > pivotsize {
                    pivot = j;
                    pivotsize = tmp;
                }
            }

            if pivotsize == 0.0 {
                return Matrix44::identity();
            }

            if pivot != i {
                // pivot i_th row and pivot_th row
                for j in 0..4 {
                    swap!(t[i][j], t[pivot][j]);
                    swap!(s[i][j], s[pivot][j]);
                }
            }

            // Elimination
            for j in (i + 1)..4 {
                let f = t[j][i] / t[i][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }
        // Backward substitution
        for i in (0..4).rev() {
            let mut f: f64 = t[i][i];

            if f == 0.0 {
                // Cannot invert singular matrix
                return Matrix44::identity();
            }

            for j in 0..4 {
                t[i][j] /= f;
                s[i][j] /= f;
            }

            for j in 0..i {
                f = t[j][i];

                for k in 0..4 {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }

        return s;
    }

    pub fn epsilon_normalized(&self, eps: Option<f64>) -> Matrix44 {
        let mut ret = self.clone();
        let eps_actual = eps.unwrap_or(1.0e-6);
        for i in 0..4 {
            for j in 0..4 {
                ret[i][j] = (ret[i][j] / eps_actual).round() * eps_actual;
            }
        }
        return ret;
    }

    pub fn det(&self) -> f64 {
        (self[0][0] * self[1][1]
            + self[2][2] * self[3][3]
            + self[0][1] * self[1][2]
            + self[2][3] * self[3][0]
            + self[0][2] * self[1][3]
            + self[2][0] * self[3][1]
            + self[0][3] * self[1][0]
            + self[2][1] * self[3][2])
            - (self[0][0] * self[3][1]
                + self[2][2] * self[1][3]
                + self[0][1] * self[3][2]
                + self[2][3] * self[1][0]
                + self[0][2] * self[3][3]
                + self[2][0] * self[1][1]
                + self[0][3] * self[3][0]
                + self[2][1] * self[1][2])
    }
}

impl Index<usize> for Matrix44 {
    type Output = [f64; 4];

    fn index(&self, idx: usize) -> &[f64; 4] {
        &self.elements[idx]
    }
}

impl IndexMut<usize> for Matrix44 {
    fn index_mut(&mut self, idx: usize) -> &mut [f64; 4] {
        &mut self.elements[idx]
    }
}

impl Mul for Matrix44 {
    type Output = Matrix44;

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn mul(self, other: Matrix44) -> Matrix44 {
        let mut result = Matrix44::identity();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[i][0] * other[0][j] +
                    self[i][1] * other[1][j] +
                    self[i][2] * other[2][j] +
                    self[i][3] * other[3][j];
            }
        }
        result
    }
}

impl Mul<Vector3> for Matrix44 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: other.x * self[0][0] + other.y * self[0][1] + other.z * self[0][2] + self[0][3],
            y: other.x * self[1][0] + other.y * self[1][1] + other.z * self[1][2] + self[1][3],
            z: other.x * self[2][0] + other.y * self[2][1] + other.z * self[2][2] + self[2][3],
        }
    }
}

impl PartialEq for Matrix44 {
    fn eq(&self, other: &Matrix44) -> bool {
        self[0][0] == other[0][0]
            && self[0][1] == other[0][1]
            && self[0][2] == other[0][2]
            && self[0][3] == other[0][3]
            && self[1][0] == other[1][0]
            && self[1][1] == other[1][1]
            && self[1][2] == other[1][2]
            && self[1][3] == other[1][3]
            && self[2][0] == other[2][0]
            && self[2][1] == other[2][1]
            && self[2][2] == other[2][2]
            && self[2][3] == other[2][3]
            && self[3][0] == other[3][0]
            && self[3][1] == other[3][1]
            && self[3][2] == other[3][2]
            && self[3][3] == other[3][3]
    }
}
