use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{length_squared, Point3};

pub(crate) struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub(crate) fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>) {
        let oc = r.origin() - self.center;
        let a = length_squared(r.direction());
        let half_b = oc.dot(r.direction());
        let c = length_squared(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return (false, None);
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return (false, None);
            }
        }

        let t = root;
        let p = r.at(t);

        let mut rec = HitRecord::new(p, (p - self.center) / self.radius, t);
        let outward_normal = (rec.p() - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        (true, Some(rec))
    }
}