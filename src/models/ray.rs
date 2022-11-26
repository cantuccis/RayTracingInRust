use super::color::Color;
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
    pub fn color(&self) -> Color {
        let unit_direction = self.direction();
        let t = 0.5f64 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
