use crate::ray::{HitRecord, Ray};
use crate::units::vec3::{dot_product, random_in_unit_sphere, refract, unit_vector};
use crate::units::{
    color::Color,
    vec3::{random_unit_vector, reflect},
};
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
#[derive(Copy, Clone, Debug, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (Color, Ray) {
        match self {
            Material::Lambertian(l) => {
                let mut scatter_direction = rec.normal + random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered = Ray::new(rec.point, scatter_direction);
                let attenuation = l.albedo; //p; // we can divide albedo / p as well.
                (attenuation, scattered)
            }
            Material::Metal(m) => {
                let reflected = reflect(&unit_vector(r_in.direction()), &(rec.normal));
                let scattered = Ray::new(rec.point, reflected + m.fuzz * random_in_unit_sphere());
                let attenuation = m.albedo;
                (attenuation, scattered)
            }
            Material::Dielectric(d) => {
                let attennuation = Color::new(1.0, 1.0, 1.0);
                let refraction_ratio = if rec.front_face { 1.0 / d.ir } else { d.ir };
                let unit_direction = unit_vector(r_in.direction());
                let neg_unit_direction = unit_direction * -1.0;
                let cos_theta = dot_product(&neg_unit_direction, &rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract {
                    reflect(&unit_direction, &rec.normal)
                } else {
                    refract(&unit_direction, &rec.normal, refraction_ratio)
                };
                let scattered = Ray::new(rec.point, direction);
                (attennuation, scattered)
            }
        }
    }
}
