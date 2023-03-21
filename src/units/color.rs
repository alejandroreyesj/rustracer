use super::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color, samples_per_pixel: i64) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    //Divide color by samples_per_pixel
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;
    r *= 255.999;
    g *= 255.999; // let x = (255.999 * pixel_color.x()) as i64;
    b *= 255.999;
    // let y = (255.999 * pixel_color.y()) as i64;
    // let z = (255.999 * pixel_color.z()) as i64;
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
