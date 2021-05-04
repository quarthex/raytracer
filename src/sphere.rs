use core::fmt::Debug;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{length_squared, Point3};

#[derive(Debug)]
pub(crate) struct Sphere<M: Material + Debug> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material + Debug> Sphere<M> {
    pub(crate) fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material + Clone + Debug> Hittable for Sphere<M> {
    type Material = M;

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<M>> {
        let oc = r.origin() - self.center;
        let a = length_squared(r.direction());
        let half_b = oc.dot(r.direction());
        let c = length_squared(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

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

        let t = root;
        let p = r.at(t);

        let mut rec = HitRecord::new(p, (p - self.center) / self.radius, self.material.clone(), t);
        let outward_normal = (rec.p() - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
