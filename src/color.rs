use crate::rtweekend::clamp;
use crate::vec3::Color;

pub(crate) fn write_color(pixel_color: &Color, samples_per_pixel: usize) {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (scale * r).sqrt();
    let g = (scale * g).sqrt();
    let b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as usize,
        (256.0 * clamp(g, 0.0, 0.999)) as usize,
        (256.0 * clamp(b, 0.0, 0.999)) as usize
    );
}
