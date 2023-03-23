use rayon::prelude::*;
use rustracer::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    ray::{self, Hittables},
    sphere::Sphere,
    units::{
        color::{write_color, Color},
        point::Point,
        vec3::{random_f64, random_f64_range, unit_vector, Vec3},
    },
};

fn main() {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let aperture = 0.1;

    // World
    let world = random_scene();

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    // Render
    println!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j} ");
        for i in 0..image_width {
            let pixel_color = Color::new(0.0, 0.0, 0.0);
            let final_color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                    let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &world, max_depth)
                })
                .reduce(|| pixel_color, |acc, x| acc + x);

            write_color(final_color, samples_per_pixel)
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

fn random_scene() -> Hittables {
    let mut world = Hittables::new();

    // Make the ground material
    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );
            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let material = Material::Lambertian(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_with_range(0.5, 1.0);
                    let fuzz = random_f64_range(0.0, 0.5);
                    let material = Material::Metal(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Material::Dielectric(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Material::Metal(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
