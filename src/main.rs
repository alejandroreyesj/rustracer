use rustracer::{
    camera::Camera,
    ray::{self, Hittables, Ray},
    sphere::Sphere,
    units::{
        color::{write_color, Color},
        point::Point,
        vec3::{random_f64, random_in_unit_sphere, unit_vector},
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
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));
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
    if let Some(rec) = world.hit(r, 0.001, std::f64::MAX) {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.point, target - rec.point), world, depth - 1);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
