use crate::vector::{Vector3D, Vector4D};

use std::fmt::Display;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3D {
    n: [Vector3D; 3],
}

impl Matrix3D {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] * self[2][2] +
        self[0][1] * self[1][2] * self[2][0] +
        self[0][2] * self[1][0] * self[2][1] -
        self[0][0] * self[1][2] * self[2][1] -
        self[0][1] * self[1][0] * self[2][2] -
        self[0][2] * self[1][1] * self[2][0]
    }

    pub fn new(n00: f64, n01: f64, n02: f64,
        n10: f64, n11: f64, n12: f64,
        n20: f64, n21: f64, n22: f64) -> Self {
            let n1 = Vector3D::new(n00, n01, n02);
            let n2 = Vector3D::new(n10, n11, n12);
            let n3 = Vector3D::new(n20, n21, n22);
            Self { n: [n1, n2, n3] }
    }

    pub fn from_vector(a: Vector3D, b: Vector3D, c: Vector3D) -> Self {
        Self { n: [a, b, c] }
    }

    pub fn identity() -> Self {
        Matrix3D::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn inverse(&self) -> Option<Matrix3D> {
        let a = self[0];
        let b = self[1];
        let c = self[2];

        let r0 = b.cross(&c);
        let r1 = c.cross(&a);
        let r2 = a.cross(&b);
        let product = r2.dot(&c);
        if product == 0.0 {
            None
        } else {
            let inv_det = 1.0 / product;
            Some(Matrix3D::new(r0.x * inv_det, r1.x * inv_det, r2.x * inv_det,
                r0.y * inv_det, r1.y * inv_det, r2.y * inv_det,
                r0.z * inv_det, r1.z * inv_det, r2.z * inv_det))
        }
    }

    pub fn make_involution(a: Vector3D) -> Matrix3D {
        Matrix3D::new(
            2.0 * a.x.powi(2) - 1.0, 2.0 * a.x * a.y, 2.0 * a.x * a.z,
            2.0 * a.x * a.y, 2.0 * a.y.powi(2) - 1.0, 2.0 * a.y * a.z,
            2.0 * a.x * a.z, 2.0 * a.y * a.z, 2.0 * a.z.powi(2) - 1.0)
    }

    pub fn make_rotation(t: f64, a: Vector3D) -> Matrix3D {
        let r = t.to_radians();
        let c = r.cos();
        let s = r.sin();
        Matrix3D::new(
            c + (1.0 - c) * a.x.powi(2), (1.0 - c) * a.x * a.y - s * a.z, (1.0 - c) * a.x * a.z + s * a.y,
            (1.0 - c) * a.x * a.y + s * a.z, c + (1.0 - c) * a.y.powi(2), (1.0 - c) * a.y * a.z - s * a.x,
            (1.0 - c) * a.x * a.z - s * a.y, (1.0 - c) * a.y * a.z + s * a.x, c + (1.0 - c) * a.z.powi(2))
    }

    pub fn make_rotation_x(t: f64) -> Matrix3D {
        let r = t.to_radians();
        let c = r.cos();
        let s = r.sin();
        Matrix3D::new(1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c)
    }

    pub fn make_rotation_y(t: f64) -> Matrix3D {
        let r = t.to_radians();
        let c = r.cos();
        let s = r.sin();
        Matrix3D::new(c, 0.0, s, 0.0, 1.0, 0.0, -s, 0.0, c)
    }

    pub fn make_rotation_z(t: f64) -> Matrix3D {
        let r = t.to_radians();
        let c = r.cos();
        let s = r.sin();
        Matrix3D::new(c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn make_reflection(a: Vector3D) -> Matrix3D {
        Matrix3D::new(
            1.0 - 2.0 * a.x.powi(2), -2.0 * a.x * a.y, -2.0 * a.x * a.z,
            -2.0 * a.x * a.y, 1.0 - 2.0 * a.y.powi(2), -2.0 * a.y * a.z,
            -2.0 * a.x * a.z, -2.0 * a.y * a.z, 1.0 - 2.0 * a.z.powi(2))
    }

    pub fn make_scale(sx: f64, sy: f64, sz: f64) -> Matrix3D {
        Matrix3D::new(sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, sz)
    }

    pub fn make_directional_scale(s: f64, a: Vector3D) -> Matrix3D {
        Matrix3D::new(
            (s - 1.0) * a.x.powi(2) + 1.0, (s - 1.0) * a.x * a.y, (s - 1.0) * a.x * a.z,
            (s - 1.0) * a.x * a.y, (s - 1.0) * a.y.powi(2) + 1.0, (s - 1.0) * a.y * a.z,
            (s - 1.0) * a.x * a.z, (s - 1.0) * a.y * a.z, (s - 1.0) * a.z.powi(2) + 1.0)
    }

    pub fn make_skew(theta: f64, a: Vector3D, b: Vector3D) -> Matrix3D {
        let t = theta.to_radians().tan();
        Matrix3D::new(
            a.x * b.x * t + 1.0, a.x * b.y * t, a.x * b.z * t,
            a.y * b.x * t, a.y * b.y * t + 1.0, a.y * b.z * t,
            a.z * b.x * t, a.z * b.y * t, a.z * b.z * t + 1.0
        )
    }
}

impl Add<Self> for Matrix3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
       Matrix3D::new(self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2],
        self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2],
        self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2])
    }
}

impl Display for Matrix3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]",
            self[0][0], self[0][1], self[0][2],
            self[1][0], self[1][1], self[1][2],
            self[2][0], self[2][1], self[2][2])
    }
}

impl Index<usize> for Matrix3D {
    type Output = Vector3D;
    fn index(&self, index: usize) -> &Self::Output {
        &self.n[index]
    }
}

impl IntoIterator for Matrix3D {
    type Item = f64;
    type IntoIter = Matrix3DIterator;

    fn into_iter(self) -> Self::IntoIter {
        Matrix3DIterator {
            n: self.n,
            index: 0,
        }
    }
}

pub struct Matrix3DIterator {
    n: [Vector3D; 3],
    index: usize,
}

impl Iterator for Matrix3DIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index / 3;
        let j = self.index % 3;
        if i < 3 && j < 3 {
            self.index += 1;
            Some(self.n[j][i])
        } else {
            None
        }
    }
}

impl Mul<f64> for Matrix3D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix3D::new(self[0][0] * rhs, self[0][1] * rhs, self[0][2] * rhs,
        self[1][0] * rhs, self[1][1] * rhs, self[1][2] * rhs,
        self[2][0] * rhs, self[2][1] * rhs, self[2][2] * rhs)
    }
}

impl Mul<Matrix3D> for f64 {
    type Output = Matrix3D;
    fn mul(self, rhs: Matrix3D) -> Self::Output {
        rhs * self
    }
}

impl Mul<Self> for Matrix3D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3D::new(
            self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0],
            self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1],
            self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2],
            self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0],
            self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1],
            self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2],
            self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0],
            self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1],
            self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2])
    }
}

impl Mul<Vector3D> for Matrix3D {
    type Output = Vector3D;
    fn mul(self, rhs: Vector3D) -> Self::Output {
        Vector3D::new(self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z) 
    }
}


impl Sub<Self> for Matrix3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
       Matrix3D::new(self[0][0] - rhs[0][0], self[0][1] - rhs[0][1], self[0][2] - rhs[0][2],
        self[1][0] - rhs[1][0], self[1][1] - rhs[1][1], self[1][2] - rhs[1][2],
        self[2][0] - rhs[2][0], self[2][1] - rhs[2][1], self[2][2] - rhs[2][2])
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4D {
    n: [Vector4D; 4],
}

impl Matrix4D {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] * self[2][2] * self[3][3] +
        self[0][0] * self[1][2] * self[2][3] * self[3][1] +
        self[0][0] * self[1][3] * self[2][1] * self[3][2] -
        self[0][0] * self[1][3] * self[2][2] * self[3][1] -
        self[0][0] * self[1][2] * self[2][1] * self[3][3] -
        self[0][0] * self[1][1] * self[2][3] * self[3][2] -
        self[0][1] * self[1][0] * self[2][2] * self[3][3] -
        self[0][2] * self[1][0] * self[2][3] * self[3][1] -
        self[0][3] * self[1][0] * self[2][1] * self[3][2] +
        self[0][3] * self[1][0] * self[2][2] * self[3][1] +
        self[0][2] * self[1][0] * self[2][1] * self[3][3] +
        self[0][1] * self[1][0] * self[2][3] * self[3][2] +
        self[0][1] * self[1][2] * self[2][0] * self[3][3] +
        self[0][2] * self[1][3] * self[2][0] * self[3][1] +
        self[0][3] * self[1][1] * self[2][0] * self[3][2] -
        self[0][3] * self[1][2] * self[2][0] * self[3][1] -
        self[0][2] * self[1][1] * self[2][0] * self[3][3] -
        self[0][1] * self[1][3] * self[2][0] * self[3][2] -
        self[0][1] * self[1][2] * self[2][3] * self[3][0] -
        self[0][2] * self[1][3] * self[2][1] * self[3][0] -
        self[0][3] * self[1][1] * self[2][2] * self[3][0] +
        self[0][3] * self[1][2] * self[2][1] * self[3][0] +
        self[0][2] * self[1][1] * self[2][3] * self[3][0] +
        self[0][1] * self[1][3] * self[2][2] * self[3][0]
    }

    pub fn new(n00: f64, n01: f64, n02: f64, n03: f64,
        n10: f64, n11: f64, n12: f64, n13: f64,
        n20: f64, n21: f64, n22: f64, n23: f64,
        n30: f64, n31: f64, n32: f64, n33: f64) -> Self {
            let n1 = Vector4D::new(n00, n01, n02, n03);
            let n2 = Vector4D::new(n10, n11, n12, n13);
            let n3 = Vector4D::new(n20, n21, n22, n23);
            let n4 = Vector4D::new(n30, n31, n32, n33);
            Self { n: [n1, n2, n3, n4] }
    }

    pub fn from_vector(a: Vector4D, b: Vector4D, c: Vector4D, d: Vector4D) -> Self {
        let n = [a, b, c, d];
        Self { n }
    }

    pub fn identity() -> Self {
        Matrix4D::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn inverse(&self) -> Option<Matrix4D> {
        let a = Vector3D::new(self[0][0], self[1][0], self[2][0]);
        let b = Vector3D::new(self[0][1], self[1][1], self[2][1]);
        let c = Vector3D::new(self[0][2], self[1][2], self[2][2]);
        let d = Vector3D::new(self[0][3], self[1][3], self[2][3]);

        let x = self[3][0];
        let y = self[3][1];
        let z = self[3][2];
        let w = self[3][3];

        let mut s = a.cross(&b);
        let mut t = c.cross(&d);
        let mut u = y * a - x * b;
        let mut v = w * c - z * d;

        let product = s.dot(&v) + t.dot(&u);
        if product == 0.0 {
            None
        } else {
            let inv_det = 1.0 / (s.dot(&v) + t.dot(&u));
            s = s * inv_det;
            t = t * inv_det;
            u = u * inv_det;
            v = v * inv_det;

            let r0 = b.cross(&v) + t * y;
            let r1 = v.cross(&a) - t * x;
            let r2 = d.cross(&u) + s * w;
            let r3 = u.cross(&c) - s * z;

            Some(Matrix4D::new(r0.x ,r0.y, r0.z, -b.dot(&t),
                r1.x ,r1.y, r1.z, a.dot(&t),
                r2.x ,r2.y, r2.z, -d.dot(&s),
                r3.x ,r3.y, r3.z, c.dot(&s)))
        }
    }

}

impl Add<Self> for Matrix4D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
       Matrix4D::new(self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2], self[0][3] + rhs[0][3],
        self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2], self[1][3] + rhs[1][3],
        self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2], self[2][3] + rhs[2][3],
        self[3][0] + rhs[3][0], self[3][1] + rhs[3][1], self[3][2] + rhs[3][2], self[3][3] + rhs[3][3])
    }
}

impl Display for Matrix4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[[{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}], [{}, {}, {}, {}]]",
            self[0][0], self[0][1], self[0][2], self[0][3],
            self[1][0], self[1][1], self[1][2], self[1][3],
            self[2][0], self[2][1], self[2][2], self[2][3],
            self[3][0], self[3][1], self[3][2], self[3][3])
    }
}

impl Index<usize> for Matrix4D {
    type Output = Vector4D;
    fn index(&self, index: usize) -> &Self::Output {
        &self.n[index]
    }
}

impl Mul<f64> for Matrix4D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix4D::new(self[0][0] * rhs, self[0][1] * rhs, self[0][2] * rhs, self[0][3] * rhs,
        self[1][0] * rhs, self[1][1] * rhs, self[1][2] * rhs, self[1][3] * rhs,
        self[2][0] * rhs, self[2][1] * rhs, self[2][2] * rhs, self[2][3] * rhs,
        self[3][0] * rhs, self[3][1] * rhs, self[3][2] * rhs, self[3][3] * rhs)
    }
}

impl Mul<Matrix4D> for f64 {
    type Output = Matrix4D;
    fn mul(self, rhs: Matrix4D) -> Self::Output {
        rhs * self
    }
}

impl Mul<Self> for Matrix4D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4D::new(
            self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0] + self[0][3] * rhs[3][0],
            self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1] + self[0][3] * rhs[3][1],
            self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2] + self[0][3] * rhs[3][2],
            self[0][0] * rhs[0][3] + self[0][1] * rhs[1][3] + self[0][2] * rhs[2][3] + self[0][3] * rhs[3][3],

            self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0] + self[1][3] * rhs[3][0],
            self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1] + self[1][3] * rhs[3][1],
            self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2] + self[1][3] * rhs[3][2],
            self[1][0] * rhs[0][3] + self[1][1] * rhs[1][3] + self[1][2] * rhs[2][3] + self[1][3] * rhs[3][3],

            self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0] + self[2][3] * rhs[3][0],
            self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1] + self[2][3] * rhs[3][1],
            self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2] + self[2][3] * rhs[3][2],
            self[2][0] * rhs[0][3] + self[2][1] * rhs[1][3] + self[2][2] * rhs[2][3] + self[2][3] * rhs[3][3],

            self[3][0] * rhs[0][0] + self[3][1] * rhs[1][0] + self[3][2] * rhs[2][0] + self[3][3] * rhs[3][0],
            self[3][0] * rhs[0][1] + self[3][1] * rhs[1][1] + self[3][2] * rhs[2][1] + self[3][3] * rhs[3][1],
            self[3][0] * rhs[0][2] + self[3][1] * rhs[1][2] + self[3][2] * rhs[2][2] + self[3][3] * rhs[3][2],
            self[3][0] * rhs[0][3] + self[3][1] * rhs[1][3] + self[3][2] * rhs[2][3] + self[3][3] * rhs[3][3]
        )
    }
}

impl Mul<Vector4D> for Matrix4D {
    type Output = Vector4D;
    fn mul(self, rhs: Vector4D) -> Self::Output {
        Vector4D::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w,
            self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w) 
    }
}

impl Sub<Self> for Matrix4D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
       Matrix4D::new(
        self[0][0] - rhs[0][0], self[0][1] - rhs[0][1], self[0][2] - rhs[0][2], self[0][3] - rhs[0][3],
        self[1][0] - rhs[1][0], self[1][1] - rhs[1][1], self[1][2] - rhs[1][2], self[1][3] - rhs[1][3],
        self[2][0] - rhs[2][0], self[2][1] - rhs[2][1], self[2][2] - rhs[2][2], self[2][3] - rhs[2][3],
        self[3][0] - rhs[3][0], self[3][1] - rhs[3][1], self[3][2] - rhs[3][2], self[3][3] - rhs[3][3])
    }
}

#[cfg(test)]
mod matrix3d_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use num_traits::Float;

    fn element_approx_eq<I: IntoIterator>(result: I, expected: I) -> ()
        where I::Item: Float,
              I::Item: std::fmt::Debug {
        for (r, e) in std::iter::zip(result, expected) {
            assert_approx_eq!(r, e, Float::epsilon());
        }
    }

    #[test]
    fn constructor() {
        let matrix = Matrix3D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9);
        assert_eq!(matrix[0][0], 0.1);
        assert_eq!(matrix[0][1], 0.2);
        assert_eq!(matrix[0][2], 0.3);
        assert_eq!(matrix[1][0], 0.4);
        assert_eq!(matrix[1][1], 0.5);
        assert_eq!(matrix[1][2], 0.6);
        assert_eq!(matrix[2][0], 0.7);
        assert_eq!(matrix[2][1], 0.8);
        assert_eq!(matrix[2][2], 0.9);
    }

    #[test]
    fn vector_constructor() {
        let vector1 = Vector3D::new(0.1, 0.2, 0.3);
        let vector2 = Vector3D::new(0.4, 0.5, 0.6);
        let vector3 = Vector3D::new(0.7, 0.8, 0.9);
        let matrix = Matrix3D::from_vector(vector1, vector2, vector3);
        assert_eq!(matrix[0][0], 0.1);
        assert_eq!(matrix[0][1], 0.2);
        assert_eq!(matrix[0][2], 0.3);
        assert_eq!(matrix[1][0], 0.4);
        assert_eq!(matrix[1][1], 0.5);
        assert_eq!(matrix[1][2], 0.6);
        assert_eq!(matrix[2][0], 0.7);
        assert_eq!(matrix[2][1], 0.8);
        assert_eq!(matrix[2][2], 0.9);
    }

    #[test]
    fn index() {
        let matrix = Matrix3D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9);
        assert_eq!(matrix[0][0], 0.1);
        assert_eq!(matrix[0][1], 0.2);
        assert_eq!(matrix[0][2], 0.3);
        assert_eq!(matrix[1][0], 0.4);
        assert_eq!(matrix[1][1], 0.5);
        assert_eq!(matrix[1][2], 0.6);
        assert_eq!(matrix[2][0], 0.7);
        assert_eq!(matrix[2][1], 0.8);
        assert_eq!(matrix[2][2], 0.9);
    }

    #[test]
    fn matrix_addition() {
        let matrix1 = Matrix3D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9);
        let matrix2 = matrix1 + matrix1;
        assert_approx_eq!(matrix2[0][0], 0.2);
        assert_approx_eq!(matrix2[0][1], 0.4);
        assert_approx_eq!(matrix2[0][2], 0.6);
        assert_approx_eq!(matrix2[1][0], 0.8);
        assert_approx_eq!(matrix2[1][1], 1.0);
        assert_approx_eq!(matrix2[1][2], 1.2);
        assert_approx_eq!(matrix2[2][0], 1.4);
        assert_approx_eq!(matrix2[2][1], 1.6);
        assert_approx_eq!(matrix2[2][2], 1.8);
    }

    #[test]
    fn matrix_subtraction() {
        let matrix1 = Matrix3D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9);
        let matrix2 = Matrix3D::new(0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8);
        let matrix3 = matrix2 - matrix1;
        assert_approx_eq!(matrix3[0][0], 0.1);
        assert_approx_eq!(matrix3[0][1], 0.2);
        assert_approx_eq!(matrix3[0][2], 0.3);
        assert_approx_eq!(matrix3[1][0], 0.4);
        assert_approx_eq!(matrix3[1][1], 0.5);
        assert_approx_eq!(matrix3[1][2], 0.6);
        assert_approx_eq!(matrix3[2][0], 0.7);
        assert_approx_eq!(matrix3[2][1], 0.8);
        assert_approx_eq!(matrix3[2][2], 0.9);
    }

    #[test]
    fn scalar_multiplication() {
        let matrix1 = Matrix3D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9);
        let matrix2 = matrix1 * 5.0;
        assert_approx_eq!(matrix2[0][0], 0.5);
        assert_approx_eq!(matrix2[0][1], 1.0);
        assert_approx_eq!(matrix2[0][2], 1.5);
        assert_approx_eq!(matrix2[1][0], 2.0);
        assert_approx_eq!(matrix2[1][1], 2.5);
        assert_approx_eq!(matrix2[1][2], 3.0);
        assert_approx_eq!(matrix2[2][0], 3.5);
        assert_approx_eq!(matrix2[2][1], 4.0);
        assert_approx_eq!(matrix2[2][2], 4.5);
        assert_eq!(matrix2, 5.0 * matrix1);
    }

    #[test]
    fn matrix_multiplication() {
        let matrix1 = Matrix3D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9);
        let matrix2 = Matrix3D::new(0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8);
        let matrix3 = matrix1 * matrix2;
        let expected = Matrix3D::new(0.6, 0.72, 0.84, 1.32, 1.62, 1.92, 2.04, 2.52, 3.0);
        assert_approx_eq!(matrix3[0][0], expected[0][0]);
        assert_approx_eq!(matrix3[0][1], expected[0][1]);
        assert_approx_eq!(matrix3[0][2], expected[0][2]);
        assert_approx_eq!(matrix3[1][0], expected[1][0]);
        assert_approx_eq!(matrix3[1][1], expected[1][1]);
        assert_approx_eq!(matrix3[1][2], expected[1][2]);
        assert_approx_eq!(matrix3[2][0], expected[2][0]);
        assert_approx_eq!(matrix3[2][1], expected[2][1]);
        assert_approx_eq!(matrix3[2][2], expected[2][2]);
    }

    #[test]
    fn matrix_vector_multiplication() {
        let matrix = Matrix3D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9);
        let vector1 = Vector3D::new(0.2, 0.4, 0.6);
        let vector2 = matrix * vector1;
        assert_approx_eq!(vector2[0], 0.1*0.2 + 0.2*0.4 + 0.3*0.6);
        assert_approx_eq!(vector2[1], 0.4*0.2 + 0.5*0.4 + 0.6*0.6);
        assert_approx_eq!(vector2[2], 0.7*0.2 + 0.8*0.4 + 0.9*0.6);
    }

    #[test]
    fn determinant() {
        let matrix = Matrix3D::new(3.0, 5.0, 6.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(matrix.determinant(), 3.0);
        let matrix_with_zero_row = Matrix3D::new(0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(matrix_with_zero_row.determinant(), 0.0);
        let identity_matrix = Matrix3D::identity();
        assert_eq!(identity_matrix.determinant(), 1.0);
        let diagonal_matrix = Matrix3D::new(2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0);
        assert_eq!(diagonal_matrix.determinant(), 8.0);
    }

    #[test]
    fn matrix_inversion() {
        let matrix = Matrix3D::new(1.0, 2.0, 3.0, 5.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let inverted_matrix = matrix.inverse().unwrap();
        let matrix_product = inverted_matrix * matrix;
        let identity_matrix = Matrix3D::identity();
        assert_approx_eq!(matrix_product[0][0], identity_matrix[0][0]);
        assert_approx_eq!(matrix_product[0][1], identity_matrix[0][1]);
        assert_approx_eq!(matrix_product[0][2], identity_matrix[0][2]);
        assert_approx_eq!(matrix_product[1][0], identity_matrix[1][0]);
        assert_approx_eq!(matrix_product[1][1], identity_matrix[1][1]);
        assert_approx_eq!(matrix_product[1][2], identity_matrix[1][2]);
        assert_approx_eq!(matrix_product[2][0], identity_matrix[2][0]);
        assert_approx_eq!(matrix_product[2][1], identity_matrix[2][1]);
        assert_approx_eq!(matrix_product[2][2], identity_matrix[2][2]);
    }

    #[test]
    fn rotation() {
        let matrix = Matrix3D::identity();
        let x_rot = Matrix3D::make_rotation_x(90.0);
        let x_expected = Matrix3D::new(1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 1.0, 0.0);
        element_approx_eq(x_rot * matrix, x_expected);
        let y_rot = Matrix3D::make_rotation_y(90.0);
        let y_expected = Matrix3D::new(0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0);
        element_approx_eq(y_rot * matrix, y_expected);
        let z_rot = Matrix3D::make_rotation_z(90.0);
        let z_expected = Matrix3D::new(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        element_approx_eq(z_rot * matrix, z_expected);
        let a = Vector3D::new(0.5_f64.sqrt(), 0.5_f64.sqrt(), 0.0);
        let a_rot = Matrix3D::make_rotation(90.0, a);
        element_approx_eq(a_rot * matrix[2], Vector3D::new(0.5_f64.sqrt(), -0.5_f64.sqrt(), 0.0));
    }

    #[test]
    fn reflection() {
        use std::f64::consts::PI;
        let v = Vector3D::new(PI/4.0, 0.0, PI/4.0);
        let a = Vector3D::new(0.0, 0.0, 1.0);
        let m_reflect = Matrix3D::make_reflection(a);
        let v_reflect = m_reflect * v;
        assert_eq!(v_reflect, Vector3D::new(PI/4.0, 0.0, -PI/4.0));
    }

    #[test]
    fn involution() {
        use std::f64::consts::PI;
        let v = Vector3D::new(PI/4.0, 0.0, PI/4.0);
        let a = Vector3D::new(0.0, 0.0, 1.0);
        let m_involution = Matrix3D::make_involution(a);
        let v_involution = m_involution * v;
        assert_eq!(v_involution, Vector3D::new(-PI/4.0, 0.0, PI/4.0));
    }

    #[test]
    fn scale() {
        let a = Matrix3D::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let s = Matrix3D::make_scale(2.0, 3.0, 4.0);
        element_approx_eq(s * a, Matrix3D::new(2.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0));
        let ds = Matrix3D::make_directional_scale(2.0, Vector3D::new(1.0, 0.0, 0.0));
        element_approx_eq(ds * a, Matrix3D::new(2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0));
        let ds = Matrix3D::make_directional_scale(2.0, Vector3D::new(0.5_f64.sqrt(), 0.5_f64.sqrt(), 0.0));
        element_approx_eq(ds * a, Matrix3D::new(2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0));
    }

    #[test]
    fn skew() {
        let a = Vector3D::new(1.0, 0.0, 0.0);
        let b = Vector3D::new(0.0, 1.0, 0.0);
        let skew = Matrix3D::make_skew(45.0, a, b);
        let m = Matrix3D::new(1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0);
        element_approx_eq(skew * m, Matrix3D::new(3.0, 3.0, 3.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0));
    }
}

#[cfg(test)]
mod matrix4d_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn constructor() {
        let matrix = Matrix4D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6);
        assert_eq!(matrix[0][0], 0.1);
        assert_eq!(matrix[0][1], 0.2);
        assert_eq!(matrix[0][2], 0.3);
        assert_eq!(matrix[0][3], 0.4);
        assert_eq!(matrix[1][0], 0.5);
        assert_eq!(matrix[1][1], 0.6);
        assert_eq!(matrix[1][2], 0.7);
        assert_eq!(matrix[1][3], 0.8);
        assert_eq!(matrix[2][0], 0.9);
        assert_eq!(matrix[2][1], 1.0);
        assert_eq!(matrix[2][2], 1.1);
        assert_eq!(matrix[2][3], 1.2);
        assert_eq!(matrix[3][0], 1.3);
        assert_eq!(matrix[3][1], 1.4);
        assert_eq!(matrix[3][2], 1.5);
        assert_eq!(matrix[3][3], 1.6);
    }

    #[test]
    fn vector_constructor() {
        let vector1 = Vector4D::new(0.1, 0.2, 0.3, 0.4);
        let vector2 = Vector4D::new(0.5, 0.6, 0.7, 0.8);
        let vector3 = Vector4D::new(0.9, 1.0, 1.1, 1.2);
        let vector4 = Vector4D::new(1.3, 1.4, 1.5, 1.6);
        let matrix = Matrix4D::from_vector(vector1, vector2, vector3, vector4);
        assert_eq!(matrix[0][0], 0.1);
        assert_eq!(matrix[0][1], 0.2);
        assert_eq!(matrix[0][2], 0.3);
        assert_eq!(matrix[0][3], 0.4);
        assert_eq!(matrix[1][0], 0.5);
        assert_eq!(matrix[1][1], 0.6);
        assert_eq!(matrix[1][2], 0.7);
        assert_eq!(matrix[1][3], 0.8);
        assert_eq!(matrix[2][0], 0.9);
        assert_eq!(matrix[2][1], 1.0);
        assert_eq!(matrix[2][2], 1.1);
        assert_eq!(matrix[2][3], 1.2);
        assert_eq!(matrix[3][0], 1.3);
        assert_eq!(matrix[3][1], 1.4);
        assert_eq!(matrix[3][2], 1.5);
        assert_eq!(matrix[3][3], 1.6);
    }

    #[test]
    fn index() {
        let matrix = Matrix4D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6);
        assert_eq!(matrix[0][0], 0.1);
        assert_eq!(matrix[0][1], 0.2);
        assert_eq!(matrix[0][2], 0.3);
        assert_eq!(matrix[0][3], 0.4);
        assert_eq!(matrix[1][0], 0.5);
        assert_eq!(matrix[1][1], 0.6);
        assert_eq!(matrix[1][2], 0.7);
        assert_eq!(matrix[1][3], 0.8);
        assert_eq!(matrix[2][0], 0.9);
        assert_eq!(matrix[2][1], 1.0);
        assert_eq!(matrix[2][2], 1.1);
        assert_eq!(matrix[2][3], 1.2);
        assert_eq!(matrix[3][0], 1.3);
        assert_eq!(matrix[3][1], 1.4);
        assert_eq!(matrix[3][2], 1.5);
        assert_eq!(matrix[3][3], 1.6);
    }

    #[test]
    fn matrix_addition() {
        let matrix1 = Matrix4D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6);
        let matrix2 = matrix1 + matrix1;
        assert_approx_eq!(matrix2[0][0], 0.2);
        assert_approx_eq!(matrix2[0][1], 0.4);
        assert_approx_eq!(matrix2[0][2], 0.6);
        assert_approx_eq!(matrix2[0][3], 0.8);
        assert_approx_eq!(matrix2[1][0], 1.0);
        assert_approx_eq!(matrix2[1][1], 1.2);
        assert_approx_eq!(matrix2[1][2], 1.4);
        assert_approx_eq!(matrix2[1][3], 1.6);
        assert_approx_eq!(matrix2[2][0], 1.8);
        assert_approx_eq!(matrix2[2][1], 2.0);
        assert_approx_eq!(matrix2[2][2], 2.2);
        assert_approx_eq!(matrix2[2][3], 2.4);
        assert_approx_eq!(matrix2[3][0], 2.6);
        assert_approx_eq!(matrix2[3][1], 2.8);
        assert_approx_eq!(matrix2[3][2], 3.0);
        assert_approx_eq!(matrix2[3][3], 3.2);
    }

    #[test]
    fn matrix_subtraction() {
        let matrix1 = Matrix4D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6);
        let matrix2 = Matrix4D::new(0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.2, 2.4, 2.6, 2.8, 3.0, 3.2);
        let matrix3 = matrix2 - matrix1;
        assert_eq!(matrix3[0][0], 0.1);
        assert_eq!(matrix3[0][1], 0.2);
        assert_eq!(matrix3[0][2], 0.3);
        assert_eq!(matrix3[0][3], 0.4);
        assert_eq!(matrix3[1][0], 0.5);
        assert_eq!(matrix3[1][1], 0.6);
        assert_eq!(matrix3[1][2], 0.7);
        assert_eq!(matrix3[1][3], 0.8);
        assert_eq!(matrix3[2][0], 0.9);
        assert_eq!(matrix3[2][1], 1.0);
        assert_eq!(matrix3[2][2], 1.1);
        assert_eq!(matrix3[2][3], 1.2);
        assert_eq!(matrix3[3][0], 1.3);
        assert_eq!(matrix3[3][1], 1.4);
        assert_eq!(matrix3[3][2], 1.5);
        assert_eq!(matrix3[3][3], 1.6);
    }

    #[test]
    fn scalar_multiplication() {
        let matrix1 = Matrix4D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6);
        let matrix2 = matrix1 * 5.0;
        assert_eq!(matrix2[0][0], 0.5);
        assert_eq!(matrix2[0][1], 1.0);
        assert_eq!(matrix2[0][2], 1.5);
        assert_eq!(matrix2[0][3], 2.0);
        assert_eq!(matrix2[1][0], 2.5);
        assert_eq!(matrix2[1][1], 3.0);
        assert_eq!(matrix2[1][2], 3.5);
        assert_eq!(matrix2[1][3], 4.0);
        assert_eq!(matrix2[2][0], 4.5);
        assert_eq!(matrix2[2][1], 5.0);
        assert_eq!(matrix2[2][2], 5.5);
        assert_eq!(matrix2[2][3], 6.0);
        assert_eq!(matrix2[3][0], 6.5);
        assert_eq!(matrix2[3][1], 7.0);
        assert_eq!(matrix2[3][2], 7.5);
        assert_eq!(matrix2[3][3], 8.0);
        assert_eq!(matrix2, 5.0 * matrix1);
    }

    #[test]
    fn matrix_multiplication() {
        let matrix1 = Matrix4D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6);
        let matrix2 = Matrix4D::new(0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8, 2.0, 2.2, 2.4, 2.6, 2.8, 3.0, 3.2);
        let matrix3 = matrix1 * matrix2;
        let expected = Matrix4D::new(1.8, 2.0, 2.2, 2.4, 4.04, 4.56, 5.08, 5.6, 6.28, 7.12, 7.96, 8.8, 8.52, 9.68, 10.84, 12.0);
        assert_approx_eq!(matrix3[0][0], expected[0][0]);
        assert_approx_eq!(matrix3[0][1], expected[0][1]);
        assert_approx_eq!(matrix3[0][2], expected[0][2]);
        assert_approx_eq!(matrix3[0][3], expected[0][3]);
        assert_approx_eq!(matrix3[1][0], expected[1][0]);
        assert_approx_eq!(matrix3[1][1], expected[1][1]);
        assert_approx_eq!(matrix3[1][2], expected[1][2]);
        assert_approx_eq!(matrix3[1][3], expected[1][3]);
        assert_approx_eq!(matrix3[2][0], expected[2][0]);
        assert_approx_eq!(matrix3[2][1], expected[2][1]);
        assert_approx_eq!(matrix3[2][2], expected[2][2]);
        assert_approx_eq!(matrix3[2][3], expected[2][3]);
        assert_approx_eq!(matrix3[3][0], expected[3][0]);
        assert_approx_eq!(matrix3[3][1], expected[3][1]);
        assert_approx_eq!(matrix3[3][2], expected[3][2]);
        assert_approx_eq!(matrix3[3][3], expected[3][3]);
    }

    #[test]
    fn matrix_vector_multiplication() {
        let matrix = Matrix4D::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6);
        let vector1 = Vector4D::new(0.2, 0.4, 0.6, 0.8);
        let vector2 = matrix * vector1;
        assert_approx_eq!(vector2[0], 0.1*0.2 + 0.2*0.4 + 0.3*0.6 + 0.4*0.8);
        assert_approx_eq!(vector2[1], 0.5*0.2 + 0.6*0.4 + 0.7*0.6 + 0.8*0.8);
        assert_approx_eq!(vector2[2], 0.9*0.2 + 1.0*0.4 + 1.1*0.6 + 1.2*0.8);
        assert_approx_eq!(vector2[3], 1.3*0.2 + 1.4*0.4 + 1.5*0.6 + 1.6*0.8);
    }

    #[test]
    fn determinant() {
        let a = Vector4D::new(1.0, 1.0, 1.0, -1.0);
        let b = Vector4D::new(1.0, 1.0, -1.0, 1.0);
        let c = Vector4D::new(1.0, -1.0, 1.0, 1.0);
        let d = Vector4D::new(-1.0, 1.0, 1.0, 1.0);
        assert_eq!(Matrix4D::from_vector(a, b, c, d).determinant(), -16.0);

        let matrix_with_zero_row = Matrix4D::new(0.0, 0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 1.0, 1.1, 1.2);
        assert_eq!(matrix_with_zero_row.determinant(), 0.0);

        let identity_matrix = Matrix4D::identity();
        assert_eq!(identity_matrix.determinant(), 1.0);

        let diagonal_matrix = Matrix4D::new(2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0);
        assert_eq!(diagonal_matrix.determinant(), 16.0);
    }

    #[test]
    fn matrix_inversion() {
        let matrix = Matrix4D::new(1.0, 1.0, 1.0, 0.0, 0.0, 3.0, 1.0, 2.0, 1.0, 0.0, 2.0, 1.0, 2.0, 3.0, 1.0, 0.0);
        let inverted_matrix = matrix.inverse().unwrap();
        let matrix_product = inverted_matrix * matrix;
        let identity_matrix = Matrix4D::identity();
        assert_approx_eq!(matrix_product[0][0], identity_matrix[0][0]);
        assert_approx_eq!(matrix_product[0][1], identity_matrix[0][1]);
        assert_approx_eq!(matrix_product[0][2], identity_matrix[0][2]);
        assert_approx_eq!(matrix_product[0][3], identity_matrix[0][3]);

        assert_approx_eq!(matrix_product[1][0], identity_matrix[1][0]);
        assert_approx_eq!(matrix_product[1][1], identity_matrix[1][1]);
        assert_approx_eq!(matrix_product[1][2], identity_matrix[1][2]);
        assert_approx_eq!(matrix_product[1][3], identity_matrix[1][3]);

        assert_approx_eq!(matrix_product[2][0], identity_matrix[2][0]);
        assert_approx_eq!(matrix_product[2][1], identity_matrix[2][1]);
        assert_approx_eq!(matrix_product[2][2], identity_matrix[2][2]);
        assert_approx_eq!(matrix_product[2][3], identity_matrix[2][3]);

        assert_approx_eq!(matrix_product[3][0], identity_matrix[3][0]);
        assert_approx_eq!(matrix_product[3][1], identity_matrix[3][1]);
        assert_approx_eq!(matrix_product[3][2], identity_matrix[3][2]);
        assert_approx_eq!(matrix_product[3][3], identity_matrix[3][3]);
    }
}