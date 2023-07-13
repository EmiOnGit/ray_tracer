use crate::{ray::Ray, vec3::Vec3, ASPECT_RATIO};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}
impl Camera {
    pub fn new() -> Camera {
        Camera::default()
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
impl Default for Camera {
    fn default() -> Self {
        let viewport_height = 2.;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.;

        let origin = Vec3::ZERO;
        let horizontal = Vec3::X * viewport_width;
        let vertical = Vec3::Y * viewport_height;
        let lower_left_corner: Vec3 =
            origin - horizontal / 2. - vertical / 2. - Vec3::Z * focal_length;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}
