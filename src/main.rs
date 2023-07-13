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
    let mut rng = fastrand::Rng::new();
    // Camera
    let camera = Camera::new();

    // Render
    println!(
        "P3
        {image_width} {image_height}
        255"
    );
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
                    let ray = camera.get_ray(u, v);
                    last + ray_color(&ray, &mut world, MAX_DEPTH)
                });
            write_color(pixel);
        }
    }
    let duration = start.elapsed();
    eprintln!("time {duration:?}");
}
fn ray_color(ray: &Ray, world: &mut World, depth: usize) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }
    if let Some(record) = world.hit(ray, 0.001, f32::MAX) {
        let target = record.point + record.normal + random_unit_vec(&mut world.rng);

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
