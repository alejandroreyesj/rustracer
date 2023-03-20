use crate::ray::Ray;

pub trait Hittable {
    type HitRecord;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Self::HitRecord>;
}
