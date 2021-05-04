use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Debug)]
pub(crate) struct HittableList<H: Hittable> {
    objects: Vec<H>,
}

impl<H: Hittable> HittableList<H> {
    pub(crate) fn new() -> Self {
        Self { objects: vec![] }
    }

    pub(crate) fn add(&mut self, object: H) {
        self.objects.push(object);
    }
}

impl<H: Hittable> Hittable for HittableList<H> {
    type Material = H::Material;

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<Self::Material>> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = *rec.t();
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}
