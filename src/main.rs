use ray_tracer::{intersectable::Intersectable, prelude::*};
use std::time::Instant;

fn main() {
    let start = Instant::now();
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
