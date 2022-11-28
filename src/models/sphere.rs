use std::rc::Rc;

use nalgebra::Vector3;

use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;

pub struct Sphere{
    center: Vector3<f64>,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Rc<dyn Material> ) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let half_b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        };
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut hit_record = HitRecord {
            t: root,
            p: ray.at(root),
            normal: (ray.at(root) - self.center) / self.radius,
            front_face: true,
            material: self.material.clone(),
        };
        hit_record.set_face_normal(&ray);
        return Some(hit_record);
    }
}
