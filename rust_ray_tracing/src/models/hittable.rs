use std::sync::Arc;

use super::{material::Material, point::Point, ray::Ray};
use nalgebra::Vector3;

pub struct HitRecord {
    pub t: f64,
    pub p: Point,
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
}
