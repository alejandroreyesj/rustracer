use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Default, Debug, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        (self.x).powi(2) + (self.y).powi(2) + (self.z).powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_vec_3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(result, v1 + v2);
    }

    #[test]
    fn add_assign_vec_3() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);

        v1 += Vec3::new(1.0, 2.0, 3.0);
        let result = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(result, v1);
    }

    #[test]
    fn sub_vec_3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let result = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(result, v1 - v2);
    }

    #[test]
    fn sub_assign_vec_3() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);

        v1 -= Vec3::new(1.0, 2.0, 3.0);
        let result = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(result, v1);
    }

    #[test]
    fn neg_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let result = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(result, -v1);
    }

    #[test]
    fn scalar_mul_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let result = Vec3::new(-2.0, -4.0, -6.0);
        assert_eq!(result, v1 * -2.0);
    }

    #[test]
    fn scalar_mul_assign_vec3() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 *= -2.0;
        let result = Vec3::new(-2.0, -4.0, -6.0);
        assert_eq!(result, v1);
    }

    #[test]
    fn scalar_div_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let result = Vec3::new(-0.5, -1.0, -1.5);
        assert_eq!(result, v1 / -2.0);
    }

    #[test]
    fn scalar_div_assign_vec3() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 /= -2.0;
        let result = Vec3::new(-0.5, -1.0, -1.5);
        assert_eq!(result, v1);
    }

    #[test]
    fn index_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1[0], 1.0)
    }

    #[test]
    fn index_mut_vec3() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1[0] = 2.0;
        assert_eq!(v1[0], 2.0)
    }

    #[test]
    fn length_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let length_squared = (v1.x).powi(2) + (v1.y).powi(2) + (v1.z).powi(2);
        let length = length_squared.sqrt();
        assert_eq!(length, v1.length())
    }
}
