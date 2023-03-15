use std::ops::{Add, Mul, MulAssign, Neg};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn z(&self) -> i32 {
        self.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
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

impl MulAssign for Vec3 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vec_3() {
        let v1 = Vec3::new(1, 2, 3);

        let v2 = Vec3::new(1, 2, 3);
        let result = Vec3::new(2, 4, 6);
        assert_eq!(result, v1 + v2);
    }
    #[test]
    fn neg_vec3() {
        let v1 = Vec3::new(1, 2, 3);

        let result = Vec3::new(-1, -2, -3);
        assert_eq!(result, -v1);
    }
}
