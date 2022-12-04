
use super::{color::Color, hittable::Hittable};
use nalgebra::Vector3;

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
    pub fn color(&self, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = world.hit(&self, 0.001, f64::MAX) {
            if let Some((scattered, attenuation)) = hit.material.scatter(self, &hit) {
                return attenuation.zip_map(&scattered.color(world, depth - 1), |l, r| l * r);
            } else {
                return Vector3::new(0.0, 0.0, 0.0);
            }
        } else {
            let unit_direction = self.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }

    
}
