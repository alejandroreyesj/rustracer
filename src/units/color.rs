use super::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let x = (255.999 * pixel_color.x()) as i64;
    let y = (255.999 * pixel_color.y()) as i64;
    let z = (255.999 * pixel_color.z()) as i64;
    println!("{x} {y} {z}");
}
