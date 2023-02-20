use nalgebra::Vector3;

use super::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.list.push(Box::new(hittable))
    }

    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut temp_record = None;

        for object in self.list.iter() {
            if let Some(hit) = object.as_ref().hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                temp_record = Some(hit);
            }
        }
        return temp_record;
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<super::aabb::AABB> {
        if self.list.is_empty() {
            return None;
        } else {
            let mut result = AABB {
                min: Vector3::zeros(),
                max: Vector3::zeros(),
            };
            let mut first_box = true;
            for object in &self.list {
                if let Some(bounding_box) = object.bounding_box(time0, time1) {
                    result = if first_box {
                        bounding_box
                    } else {
                        surrounding_box(&result, &bounding_box)
                    };
                    first_box = false;
                } else {
                    return None;
                }
            }
            Some(result)
        }
    }
}
