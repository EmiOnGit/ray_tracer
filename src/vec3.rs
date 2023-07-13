use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0., 0., 0.);
    pub const X: Vec3 = Vec3::new(1., 0., 0.);
    pub const Y: Vec3 = Vec3::new(0., 1., 0.);
    pub const Z: Vec3 = Vec3::new(0., 0., 1.);
    pub const ONE: Vec3 = Vec3::new(1., 1., 1.);

    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn unit(&self) -> Vec3 {
        *self * self.length().recip()
    }
    pub fn sqrt(&self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }
}
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        self * (1. / rhs)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn length() {
        assert_eq!(Vec3::X.length(), 1.);
        assert_eq!(Vec3::Y.length(), 1.);
        assert_eq!(Vec3::Z.length(), 1.);
        assert_eq!(Vec3::ZERO.length(), 0.);

        assert_eq!(Vec3::new(1., -2., 2.).length(), 3.);
    }
    #[test]
    fn dot() {
        let x = Vec3::X;
        assert_eq!(x.dot(&Vec3::X), 1.);
        assert_eq!(x.dot(&Vec3::ZERO), 0.);
        let y = Vec3::new(4., 1., 5.);
        assert_eq!(x.dot(&y), 4.);
    }
    #[test]
    fn unit() {
        let x = Vec3::new(10., 0., 0.);
        assert_eq!(x.unit(), Vec3::X);
        let x = Vec3::new(28., 0., 28.);
        assert_eq!(x.unit(), f32::sqrt(2.) / 2. * (Vec3::X + Vec3::Z));

        assert!((x.unit().length() - 1.).abs() < 0.001);
    }
}
