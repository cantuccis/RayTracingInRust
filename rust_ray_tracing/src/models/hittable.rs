use std::sync::Arc;

use super::{material::Material, point::Point, ray::Ray, aabb::AABB};
use nalgebra::Vector3;

pub struct HitRecord {
    pub t: f64,
    pub p: Point,
    pub u: f64,
    pub v: f64,
    pub normal: Vector3<f64>,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}


impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = r.direction().dot(&self.normal) < 0.0;
        self.normal = if self.front_face {
            self.normal
        } else {
            -self.normal
        };
    }
}
pub trait Hittable : Sync + Send{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0:f64, time1:f64) ->  Option<AABB>;
}

pub struct FlipNormals<H: Hittable> {
    hitable: H
}

impl<H: Hittable> FlipNormals<H> {
    pub fn new(hitable: H) -> Self { FlipNormals { hitable } }
}

impl<H: Hittable> Hittable for FlipNormals<H> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hitable.hit(&ray, t_min, t_max).map(|mut hit| {
            hit.normal = -hit.normal;
            hit
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> { self.hitable.bounding_box(t0, t1) }
}