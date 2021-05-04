use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub(crate) struct HitRecord<M: Material> {
    p: Point3,
    normal: Vec3,
    material: M,
    t: f64,
    front_face: bool,
}

impl<M: Material> HitRecord<M> {
    pub(crate) fn new(p: Point3, normal: Vec3, material: M, t: f64) -> Self {
        Self {
            p,
            normal,
            material,
            t,
            front_face: false,
        }
    }

    pub(crate) fn p(&self) -> &Point3 {
        &self.p
    }

    pub(crate) fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub(crate) fn material(&self) -> &M {
        &self.material
    }

    pub(crate) fn t(&self) -> &f64 {
        &self.t
    }

    pub(crate) fn front_face(&self) -> &bool {
        &self.front_face
    }

    pub(crate) fn set_p(&mut self, p: Point3) {
        self.p = p;
    }

    pub(crate) fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub(crate) fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    pub(crate) fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}

pub(crate) trait Hittable {
    type Material: Material;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<Self::Material>>;
}
