use nalgebra::Vector3;

use super::{point3::Point3, ray::Ray};

pub struct Camera {
    origin: Point3,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64) -> Self {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
