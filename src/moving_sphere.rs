use core::fmt::Debug;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::scene_loader::StartEndPair;
use crate::vec3::{length_squared, Point3};

#[derive(Debug)]
pub(crate) struct MovingSphere<M: Material + Debug> {
    center: StartEndPair<Point3>,
    time: StartEndPair<f64>,
    radius: f64,
    material: M,
}

impl<M: Material + Debug> MovingSphere<M> {
    pub(crate) fn new(
        center: StartEndPair<Point3>,
        time: StartEndPair<f64>,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center,
            time,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center.start()
            + ((time - self.time.start()) / (self.time.end() - self.time.start()))
                * (self.center.end() - self.center.start())
    }
}

impl<M: Material + Clone + Debug> Hittable for MovingSphere<M> {
    type Material = M;

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<Self::Material>> {
        let oc = r.origin() - self.center(r.time());
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

        let mut rec = HitRecord::new(
            p,
            (p - self.center(r.time())) / self.radius,
            self.material.clone(),
            t,
        );
        let outward_normal = (rec.p() - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
