use crate::ray::Ray;

use super::{point::Point, vec3::Vec3};

pub trait Hittable {
    fn hit(self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool;
}

struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f64,
}
