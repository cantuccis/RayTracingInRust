use super::{color::Color, point3::Point3};
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
        let mut t = self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let N = (self.at(t) - Vector3::new(0.0, 0.0, -1.0)).normalize();
            return 0.5 * Color::new(N.x + 1.0, N.y + 1.0, N.z + 1.0);
        }
        let unit_direction = self.direction();
        t = 0.5f64 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }

    fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = self.origin() - center;
        let a = self.direction().dot(&self.direction());
        let b = 2.0 * oc.dot(&self.direction());
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - f64::sqrt(discriminant)) / 2.0 * a
        }
    }
}
