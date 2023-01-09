use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn cross(&self, rhs: &Vector3D) -> Self {
        Vector3D { x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x }
    }

    pub fn dot(&self, rhs: &Vector3D) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn magnitude(&self) -> f64 {
        let sum = f64::powi(self.x, 2) + f64::powi(self.y, 2) + f64::powi(self.z, 2);
        sum.sqrt()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(&self) -> Vector3D {
        let magnitude = self.magnitude();
        *self / magnitude
    }

    pub fn project(&self, rhs: &Vector3D) -> Vector3D {
        *rhs * (self.dot(&rhs) / rhs.dot(&rhs))
    }

    pub fn reject(&self, rhs: &Vector3D) -> Vector3D {
        *self - self.project(rhs)
    }
}

impl Add<Self> for Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Div<f64> for Vector3D {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3D::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Index<usize> for Vector3D {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} out of range", index),
        }
    }
}

impl Mul<f64> for Vector3D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3D::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;
    fn mul(self, rhs: Vector3D) -> Self::Output {
        rhs * self
    }
}

impl Neg for Vector3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

impl Sub<Self> for Vector3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn index_operator() {
        let vector = Vector3D::new(1.1, 2.2, 3.3);
        let mut sum: f64 = 0.0;
        for i in 0..3 {
            sum += vector[i];
        }
        assert_eq!(sum, 6.6);
    }

    #[test]
    fn scalar_multiplication() {
        let vector = Vector3D::new(3.3, 6.6, 7.7);
        let new_vector = vector * 2.2;
        assert_eq!(new_vector.x, 7.26);
        assert_eq!(new_vector.y, 14.52);
        assert_eq!(new_vector.z, 16.94);
        assert_eq!(new_vector, 2.2 * vector);
    }

    #[test]
    fn scalar_division() {
        let vector = Vector3D::new(7.26, 14.52, 16.94);
        let new_vector = vector / 2.2;
        assert_eq!(new_vector.x, 3.3);
        assert_eq!(new_vector.y, 6.6);
        assert_eq!(new_vector.z, 7.7);
    }

    #[test]
    fn negation() {
        let vector = Vector3D::new(2.2, 2.2, 2.2);
        let new_vector = -vector;
        assert_eq!(new_vector.x, -2.2);
        assert_eq!(new_vector.y, -2.2);
        assert_eq!(new_vector.z, -2.2);
    }

    #[test]
    fn magnitude() {
        let vector = Vector3D::new(2.0, 2.0, 2.0);
        let magnitude = vector.magnitude();
        let expected = 3.46410161514;
        assert_approx_eq!(magnitude, expected);
    }

    #[test]
    fn normalize() {
        let vector = Vector3D::new(1.1, 1.1, 1.1);
        let normalized_vector = vector.normalize();
        assert_eq!(normalized_vector.magnitude(), 1.0);
    }

    #[test]
    fn vector_addition() {
        let vector1 = Vector3D::new(1.1, 1.1, 1.1);
        let vector2 = Vector3D::new(2.2, 2.2, 2.2);
        let vector3 = vector1 + vector2;
        assert_approx_eq!(vector3.x, 3.3);
        assert_approx_eq!(vector3.y, 3.3);
        assert_approx_eq!(vector3.z, 3.3);
    }

    #[test]
    fn vector_subtraction() {
        let vector1 = Vector3D::new(2.2, 2.2, 2.2);
        let vector2 = Vector3D::new(1.1, 1.1, 1.1);
        let vector3 = vector1 - vector2;
        assert_approx_eq!(vector3.x, 1.1);
        assert_approx_eq!(vector3.y, 1.1);
        assert_approx_eq!(vector3.z, 1.1);
    }

    #[test]
    fn dot_product() {
        let vector1 = Vector3D::new(2.2, 2.2, 2.2);
        let vector2 = Vector3D::new(1.1, 1.1, 1.1);
        let product = vector1.dot(&vector2);
        assert_approx_eq!(product, 7.26);

        let squared_magnitude = vector1.dot(&vector1);
        assert_approx_eq!(squared_magnitude, f64::powi(vector1.magnitude(), 2));
    }

    #[test]
    fn cross_product() {
        let vector1 = Vector3D::new(0.1, 0.2, 0.3);
        let vector2 = Vector3D::new(0.4, 0.5, 0.6);
        let product = vector1.cross(&vector2);
        assert_approx_eq!(product.x, -0.03);
        assert_approx_eq!(product.y, 0.06);
        assert_approx_eq!(product.z, -0.03);

        let vector1 = Vector3D::new(1.0, 1.0, 1.0);
        let vector2 = Vector3D::new(-1.0, -1.0, -1.0);
        let vector3 = Vector3D::new(5.0, 5.0, 5.0);
        let zero_vector = Vector3D::new(0.0, 0.0, 0.0);
        assert_eq!(vector1.cross(&vector2), zero_vector);
        assert_eq!(vector1.cross(&vector3), zero_vector);
        assert_eq!(vector1.cross(&vector1), zero_vector);

        let vector1 = Vector3D::new(1.5, -1.5, 1.5);
        let vector2 = Vector3D::new(-2.3, 3.3, -5.6);
        let product = vector1.cross(&vector2);
        assert_approx_eq!(vector1.dot(&product), 0.0);
    }

    #[test]
    fn projection() {
        use std::f64::consts::PI;
        let vector1 = Vector3D::new(PI/4.0, PI/4.0, PI/4.0);
        let i = Vector3D::new(1.0, 0.0, 0.0);
        let j = Vector3D::new(0.0, 1.0, 0.0);
        let k = Vector3D::new(0.0, 0.0, 1.0);
        let zero_vector = Vector3D::new(0.0, 0.0, 0.0);
        assert_eq!(vector1.x, vector1.project(&i).x);
        assert_eq!(vector1.y, vector1.project(&j).y);
        assert_eq!(vector1.z, vector1.project(&k).z);
        assert_eq!(i, i.project(&i));
        assert_eq!(zero_vector, i.project(&j));
    }

    #[test]
    fn rejection() {
        use std::f64::consts::PI;
        let vector1 = Vector3D::new(PI/4.0, PI/4.0, PI/4.0);
        let i = Vector3D::new(1.0, 0.0, 0.0);
        let j = Vector3D::new(0.0, 1.0, 0.0);
        let k = Vector3D::new(0.0, 0.0, 1.0);
        let zero_vector = Vector3D::new(0.0, 0.0, 0.0);
        let rejected_i = vector1.reject(&i);
        assert_eq!(rejected_i.x, 0.0);
        assert_eq!(rejected_i.y, PI/4.0);
        assert_eq!(rejected_i.z, PI/4.0);
        assert_eq!(0.0, vector1.reject(&k).z);
        assert_eq!(i.reject(&i), zero_vector);
        assert_eq!(i, i.reject(&j));
        assert_eq!(vector1, vector1.project(&i) + vector1.reject(&i));
    }
}
