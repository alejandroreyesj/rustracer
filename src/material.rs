use crate::ray::{HitRecord, Ray};
use crate::units::vec3::{random_in_unit_sphere, unit_vector};
use crate::units::{
    color::Color,
    vec3::{random_unit_vector, reflect},
};
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    // Dielectric(Dielectric),
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
            } // Material::Dielectric(d) => d.scatter(r_in, rec),
        }
    }
}