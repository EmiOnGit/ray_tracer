use std::{f32::consts::PI, iter::repeat_with};

use crate::vec3::Vec3;

pub fn degrees_to_radians(degree: f32) -> f32 {
    degree * PI / 180.
}
pub fn random_in_unit_sphere(rand: &mut fastrand::Rng) -> Vec3 {
    repeat_with(|| (rand.f32(), rand.f32(), rand.f32()))
        .map(|(r1, r2, r3)| {
            let p = Vec3::new(r1, r2, r3);
            (2. * p) - Vec3::ONE
        })
        .find(|p| p.length_squared() < 1.)
        .unwrap()
}

pub fn random_unit_vec(rand: &mut fastrand::Rng) -> Vec3 {
    random_in_unit_sphere(rand).unit()
}
