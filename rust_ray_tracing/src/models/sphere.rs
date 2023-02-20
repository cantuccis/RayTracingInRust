use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::material::Material;
use super::ray::Ray;
use nalgebra::Vector3;
use std::sync::Arc;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;

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
        let normal = (ray.at(root) - self.center) / self.radius;
        let (u, v) = get_sphere_uv(&normal);
        let mut hit_record = HitRecord {
            t: root,
            p: ray.at(root),
            u,
            v,
            normal,
            front_face: true,
            material: self.material.clone(),
        };
        hit_record.set_face_normal(&ray);
        return Some(hit_record);
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        ))
    }
}

fn get_sphere_uv(p: &Vector3<f64>) -> (f64, f64) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
    let v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;
    (u, v)
}
