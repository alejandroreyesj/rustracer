use std::vec;

use rustracer::{
    ray,
    units::{color, point, vec3},
};
fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let world = vec![
        Sphere::new(point::Point::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(point::Point::new(0.0, -100.5, -1.0), 100.0),
    ];
    // Camera
    let viewport_height = 2.0;
    let view_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point::Point::new(0.0, 0.0, 0.0);
    let horizontal = vec3::Vec3::new(view_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3::Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = ray::Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = ray_color(&ray, &world);

            color::write_color(color);
        }
    }
    eprintln!("Done");
}
fn ray_color(r: &ray::Ray) -> color::Color {
    if hit_sphere(point::Point::new(0.0, 0.0, -1.0), 0.5, r) {
        return color::Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    color::Color::new(1.0, 1.0, 1.0) * (1.0 - t) + color::Color::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: point::Point, radius: f64, r: &ray::Ray) -> bool {
    let oc = r.origin() - center;
    let a = vec3::dot_product(&r.direction(), &r.direction());
    let b = vec3::dot_product(&oc, &(r.direction())) * 2.0;
    let c = vec3::dot_product(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

#[allow(dead_code)]
fn ppm_image() {
    let image_width = 1024;
    let image_height = 1024;
}
