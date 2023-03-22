use crate::{
    material::Material,
    ray::{HitRecord, Hittable, Ray},
    units::{point::Point, vec3::dot_product},
};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot_product(&oc, &(ray.direction()));
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let outward_normal = (point - self.center) / self.radius;
        let mut record = HitRecord::new(point, normal, root);
        record.set_face_normal(ray, &outward_normal);
        record.material = self.material;
        Some(record)
    }
}
