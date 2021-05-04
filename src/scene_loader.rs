use std::io::prelude::*;

use color_eyre::eyre::Result;
use serde::Deserialize;

use crate::hittable::HitRecord;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal, Scatter};
use crate::moving_sphere::MovingSphere;
use crate::sphere::Sphere;
use crate::Hittable;
use crate::Ray;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Color {
    r: f64,
    g: f64,
    b: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Material {
    Metal { albedo: Color, fuzz: f64 },
    Lambertian { albedo: Color },
    Dielectric { ir: f64 },
}

impl crate::Material for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord<Self>, scattered: &Ray) -> Scatter {
        match self {
            Self::Metal { albedo, fuzz } => {
                let albedo = crate::Color::new(albedo.r, albedo.g, albedo.b);
                let material = Metal::new(albedo, *fuzz);
                let rec = HitRecord::new(*rec.p(), *rec.normal(), material.clone(), *rec.t());
                material.scatter(r_in, &rec, scattered)
            }
            Self::Lambertian { albedo } => {
                let albedo = crate::Color::new(albedo.r, albedo.g, albedo.b);
                let material = Lambertian::new(albedo);
                let rec = HitRecord::new(*rec.p(), *rec.normal(), material.clone(), *rec.t());
                material.scatter(r_in, &rec, scattered)
            }
            Self::Dielectric { ir } => {
                let material = Dielectric::new(*ir);
                let rec = HitRecord::new(*rec.p(), *rec.normal(), material.clone(), *rec.t());
                material.scatter(r_in, &rec, scattered)
            }
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub(crate) struct StartEndPair<T> {
    start: T,
    end: T,
}

impl<T> StartEndPair<T> {
    pub(crate) fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub(crate) fn start(&self) -> &T {
        &self.start
    }

    pub(crate) fn end(&self) -> &T {
        &self.end
    }
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum Object {
    Sphere {
        center: Point3,
        radius: f64,
        material: Material,
    },
    MovingSphere {
        center: StartEndPair<Point3>,
        time: StartEndPair<f64>,
        radius: f64,
        material: Material,
    },
}

impl Hittable for Object {
    type Material = Material;
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<Self::Material>> {
        match self {
            Self::Sphere {
                center,
                radius,
                material,
            } => {
                let center = crate::Point3::new(center.x, center.y, center.z);
                Sphere::new(center, *radius, material.clone()).hit(r, t_min, t_max)
            }
            Self::MovingSphere {
                center,
                time,
                radius,
                material,
            } => {
                let center = StartEndPair {
                    start: crate::vec3::Point3::new(center.start.x, center.start.y, center.start.z),
                    end: crate::vec3::Point3::new(center.end.x, center.end.y, center.end.z),
                };
                MovingSphere::new(center, time.clone(), *radius, material.clone())
                    .hit(r, t_min, t_max)
            }
        }
    }
}

pub(crate) fn load_scene(path: &str) -> Result<HittableList<Object>> {
    let mut scene_yml;

    if path == "-" {
        scene_yml = String::new();

        std::io::stdin().read_to_string(&mut scene_yml)?;
    } else {
        scene_yml = std::fs::read_to_string(path)?;
    }

    let scene = serde_yaml::from_str::<Vec<Object>>(&scene_yml)?;
    let mut world = HittableList::new();
    for object in scene {
        world.add(object);
    }

    Ok(world)
}
