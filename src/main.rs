use ray_tracer::{
    color::write_color, intersectable::Intersectable, math::random_unit_vec, prelude::*,
};
use std::{iter::repeat_with, time::Instant};

fn main() {
    let start = Instant::now();
    let image_height = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    let image_width = IMAGE_WIDTH;
    // World
    let mut world = World::default();
    world.push(Intersectable::Sphere {
        center: Vec3::new(0., 0., -1.),
        radius: 0.5,
    });
    world.push(Intersectable::Sphere {
        center: Vec3::new(0., -100.5, -1.),
        radius: 100.,
    });
    world.render();
    let duration = start.elapsed();
    eprintln!("time {duration:?}");
}
