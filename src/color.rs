use crate::{vec3::Vec3, SAMPLES_PER_PIXEL};

pub type Color = Vec3;
pub fn write_color(color: Color) {
    let color = (color / SAMPLES_PER_PIXEL as f32).sqrt();
    let r = (255.999 * color.x.clamp(0., 0.999)) as u8;
    let g = (255.999 * color.y.clamp(0., 0.999)) as u8;
    let b = (255.999 * color.z.clamp(0., 0.999)) as u8;
    println!("{r} {g} {b}");
}
