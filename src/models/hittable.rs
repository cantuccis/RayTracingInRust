use std::rc::Rc;

use super::{material::Material, point3::Point3, ray::Ray};
use nalgebra::Vector3;

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vector3<f64>,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}


// impl<'a> Clone for HitRecord<'a> {
//     fn clone(&self) -> HitRecord<'a> {
//         HitRecord {
//             t: self.t.to_owned(),
//             p: self.p.to_owned(),
//             normal: self.normal.to_owned(),
//             front_face: self.front_face.to_owned(),
//             material: Box::new(self.material.as_mut())
//         }
//     }
// }

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = r.direction().dot(&self.normal) > 0.0;
        self.normal = if self.front_face {
            -1.0 * &self.normal
        } else {
            self.normal
        };
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
