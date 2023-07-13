pub mod camera;
pub mod color;
pub mod intersectable;
pub mod math;
pub mod ray;
pub mod vec3;
pub mod world;

pub const IMAGE_WIDTH: usize = 1024;
pub const ASPECT_RATIO: f32 = 16. / 9.;
pub const SAMPLES_PER_PIXEL: usize = 124;
pub const MAX_DEPTH: usize = 64;

pub mod prelude {
    pub use crate::{
        camera::Camera,
        color::Color,
        intersectable::{HitRecord, Intersectable},
        ray::Ray,
        vec3::Vec3,
        world::World,
        ASPECT_RATIO, IMAGE_WIDTH, MAX_DEPTH, SAMPLES_PER_PIXEL,
    };
}
