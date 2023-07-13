use crate::{ray::Ray, vec3::Vec3};
pub enum Intersectable {
    Sphere { center: Vec3, radius: f32 },
}

impl Intersectable {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let Intersectable::Sphere { center, radius } = self;
        let oc = ray.origin - *center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let point = ray.at(root);
        let outward_normal = (point - *center) / *radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -1. * outward_normal
        };
        Some(HitRecord {
            point,
            t: root,
            normal,
            front_face,
        })
    }
}
#[derive(PartialEq, Debug, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1. * outward_normal
        };
    }
}
