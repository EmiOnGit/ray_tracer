use crate::{
    intersectable::{HitRecord, Intersectable},
    ray::Ray,
};

#[derive(Default)]
pub struct World {
    pub objects: Vec<Intersectable>,
    pub rng: fastrand::Rng,
}

impl World {
    pub fn push(&mut self, obj: Intersectable) {
        self.objects.push(obj);
    }
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_distance = t_max;

        for hittable in &self.objects {
            let result = hittable.hit(ray, t_min, closest_distance);
            if let Some(record) = result {
                closest_hit = Some(record.clone());
                closest_distance = record.t;
            }
        }
        closest_hit
    }
}
