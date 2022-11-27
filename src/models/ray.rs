use super::{color::Color, point3::Point3, hittable::Hittable};
use nalgebra::Vector3;
use rand::Rng;

pub struct Ray {
    a: Vector3<f64>,
    b: Vector3<f64>,
}
impl Ray {
    pub fn new(a: Vector3<f64>, b: Vector3<f64>) -> Self {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vector3<f64> {
        self.a
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.b
    }
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.a + t * self.b
    }
    pub fn color(&self, world : &dyn Hittable) -> Color {
        if let Some(hit) = world.hit(&self, 0.0, f64::MAX) {
            0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = self.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }   
    }

    fn random_in_unit_sphere() -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        let unit = Vector3::new(1.0, 1.0, 1.0);
        loop {
            let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
            if p.magnitude_squared() < 1.0 {
                return p
            }
        }
    }

}
