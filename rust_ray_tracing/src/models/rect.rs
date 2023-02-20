use std::sync::Arc;

use nalgebra::Vector3;

use super::{hittable::{Hittable, HitRecord}, material::Material, ray::Ray, aabb::AABB};

pub enum Plane {
    YZ,
    ZX,
    XY
}

pub struct AARect {
    plane: Plane,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    material: Arc<dyn Material>,
}

impl AARect {
    pub fn new(plane: Plane, a0: f64, a1: f64, b0: f64, b1: f64, k: f64,  material: Arc<dyn Material>) -> Self {
        AARect { plane, a0, a1, b0, b1, k, material }
    }
}

impl Hittable for AARect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis) = match &self.plane {
            Plane::YZ => (0, 1, 2),
            Plane::ZX => (1, 2, 0),
            Plane::XY => (2, 0, 1)
        };
        let t = (self.k - ray.origin()[k_axis]) / ray.direction()[k_axis];
        if t < t_min || t > t_max {
            None
        } else {
            let a = ray.origin()[a_axis] + t * ray.direction()[a_axis];
            let b = ray.origin()[b_axis] + t * ray.direction()[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let p = ray.at(t);
                let mut normal = Vector3::zeros();
                normal[k_axis] = 1.0;
                Some(HitRecord { t, u, v, p, normal, material: self.material.clone(), front_face:true })
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let min = Vector3::new(self.a0, self.b0, self.k - 0.0001);
        let max = Vector3::new(self.a1, self.b1, self.k + 0.0001);
        Some(AABB { min, max })
    }
}