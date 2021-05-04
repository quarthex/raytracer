use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::vec3::{
    near_zero, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color,
};

pub(crate) type Scatter = Option<(Ray, Color)>;

pub(crate) trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<Self>, scattered: &Ray) -> Scatter
    where
        Self: std::marker::Sized;
}

#[derive(Clone, Debug)]
pub(crate) struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub(crate) fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<Self>, _scatteredd: &Ray) -> Scatter {
        let mut scatter_direction = rec.normal() + random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = *rec.normal();
        }

        Some((
            Ray::new(*rec.p(), scatter_direction, Some(r_in.time())),
            self.albedo,
        ))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub(crate) fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<Self>, _scatter: &Ray) -> Scatter {
        let reflected = reflect(&unit_vector(r_in.direction()), rec.normal());

        // The book has this surrounded by something like:
        //
        //     if scatter.direction().dot(rec.normal()) > 0.0 {
        //
        // but for some reason, that makes my metal spheres appear black.
        //
        // Everything *seems* to work fine without it.

        return Some((
            Ray::new(
                *rec.p(),
                reflected + self.fuzz * random_in_unit_sphere(),
                Some(r_in.time()),
            ),
            self.albedo,
        ));
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Dielectric {
    ir: f64, // Index of refraction
}

impl Dielectric {
    pub(crate) fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);

        r0 + (1.0 + r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<Self>, _scattered: &Ray) -> Scatter {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if *rec.front_face() {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = (-unit_direction).dot(rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
            direction = reflect(&unit_direction, rec.normal());
        } else {
            direction = refract(&unit_direction, rec.normal(), refraction_ratio);
        }

        Some((
            Ray::new(*rec.p(), direction, Some(r_in.time())),
            attenuation,
        ))
    }
}
