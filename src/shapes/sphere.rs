use crate::{
    ray::Ray,
    units::{
        hittable::Hittable,
        point::Point,
        vec3::{dot_product, Vec3},
    },
};

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    type HitRecord = HitRecord;
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Self::HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot_product(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < c {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find nearest root in acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let mut record = HitRecord::new(t, p, outward_normal, false);
        record.set_face_normal(r, &outward_normal);
        Some(record)
    }
}

#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
pub struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Point, normal: Vec3, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot_product(&(r.direction()), outward_normal) < 0.0;
        if self.front_face {
            return;
        }
        self.normal *= -1.0
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> Point {
        self.p
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

#[cfg(test)]
mod test {
    use crate::units::vec3::Vec3;

    use super::HitRecord;

    #[test]
    fn test_set_face_normal() {
        let mut record = HitRecord::new(
            0.0,
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(0.0, 0.0, 2.0),
            false,
        );
        let r = crate::ray::Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        record.set_face_normal(&r, &outward_normal);
        assert_eq!(record.normal, Vec3::new(0.0, 0.0, -2.0));
    }
}
