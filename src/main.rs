use rustracer::{
    camera::Camera,
    material::{Lambertian, Material, Metal},
    ray::{self, Hittables},
    sphere::Sphere,
    units::{
        color::{write_color, Color},
        point::Point,
        vec3::{random_f64, unit_vector},
    },
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = Hittables::new();

    let material_ground = Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Material::Metal(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }

            write_color(pixel_color, samples_per_pixel)
        }
    }
    eprintln!("Done");
}

fn ray_color(r: &ray::Ray, world: &Hittables, depth: i32) -> Color {
    if depth < 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let (attenuation, scattered) = rec.material.scatter(r, &rec);
        return ray_color(&scattered, world, depth - 1) * attenuation;
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
