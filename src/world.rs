use std::iter::repeat_with;

use crate::{
    color::write_color,
    intersectable::{HitRecord, Intersectable},
    math::random_unit_vec,
    prelude::{Camera, Color},
    ray::Ray,
    ASPECT_RATIO, IMAGE_WIDTH, MAX_DEPTH, SAMPLES_PER_PIXEL,
};

#[derive(Default)]
pub struct World {
    pub objects: Vec<Intersectable>,
    pub rng: fastrand::Rng,
    pub camera: Camera,
}

impl World {
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
    pub fn push(&mut self, obj: Intersectable) {
        self.objects.push(obj);
    }
    pub fn render(&mut self) {
        let image_height = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
        let image_width = IMAGE_WIDTH;
        println!(
            "P3
        {image_width} {image_height}
        255"
        );
        let mut rng = self.rng.fork();
        for j in 0..image_height {
            let percent = (j as f32 / image_height as f32 * 100.) as i8;
            eprintln!("{percent}%");
            let j = image_height - 1 - j;
            for i in 0..image_width {
                let pixel = repeat_with(|| (rng.f32(), rng.f32()))
                    .take(SAMPLES_PER_PIXEL)
                    .fold(Color::ZERO, |last, (rand, rand2)| {
                        let u = (i as f32 + rand) / (image_width - 1) as f32;
                        let v = (j as f32 + rand2) / (image_height - 1) as f32;
                        let ray = self.camera.get_ray(u, v);
                        last + ray_color(&ray, &self, MAX_DEPTH)
                    });
                write_color(pixel);
            }
        }
    }
}
fn ray_color(ray: &Ray, world: &World, depth: usize) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }
    if let Some(record) = world.hit(ray, 0.001, f32::MAX) {
        // TODO PERF: Shouldn't create a new rng for every ray
        let mut rng = fastrand::Rng::default();
        let target = record.point + record.normal + random_unit_vec(&mut rng);

        return 0.5
            * ray_color(
                &Ray::new(record.point, target - record.point),
                world,
                depth - 1,
            );
    }
    let unit_direction = ray.direction.unit();
    let lambda = 0.5 * (unit_direction.y + 1.);
    (1. - lambda) * Color::new(1., 1., 1.) + lambda * Color::new(0.5, 0.7, 1.)
}
