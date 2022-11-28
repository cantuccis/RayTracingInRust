use nalgebra::Vector3;

use crate::random_vector_in_unit_disk;

use super::{point3::Point3, ray::Ray};
use std::f64;

pub struct Camera {
    origin: Point3,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
    w: Vector3<f64>,

}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vector3<f64>,
        vertical_field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64
    ) -> Self {
        let theta = vertical_field_of_view * f64::consts::PI / 180.0;
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;
        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_vector_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
