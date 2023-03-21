use super::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i64) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    //Divide color by samples_per_pixel
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale * 256.0 * clamp(r, 0.0, 0.999);
    g *= scale * 256.0 * clamp(r, 0.0, 0.999);
    b *= scale * 256.0 * clamp(r, 0.0, 0.999);

    println!("{} {} {}", r as i64, g as i64, b as i64);
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
