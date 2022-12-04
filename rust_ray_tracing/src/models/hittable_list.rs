use super::{
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

    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self { HittableList { list } }
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
}
