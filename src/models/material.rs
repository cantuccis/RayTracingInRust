use nalgebra::Vector3;
use rand::Rng;

use crate::random_vector_in_unit_sphere;

use super::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material: Sync {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + random_vector_in_unit_sphere();
        let is_near_0 = scatter_direction.x.abs() < 1e-8
            && scatter_direction.y.abs() < 1e-8
            && scatter_direction.z.abs() < 1e-8;
        if is_near_0 {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.p, scatter_direction);
        return Some((scattered, self.albedo));
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let fuzz = if self.fuzz < 1.0 { 1.0 } else { self.fuzz };

        let reflected = reflect(ray_in.direction().normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + fuzz * random_vector_in_unit_sphere(),
        );
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            return Some((scattered, self.albedo));
        } else {
            return None;
        }
    }
}

pub struct Dielectric {
    pub index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let (outward_normal, etai_over_etat, cosine) = if ray_in.direction().dot(&hit_record.normal) > 0.0 {
            let cosine = self.index_of_refraction * ray_in.direction().dot(&hit_record.normal) / ray_in.direction().magnitude();
            (-hit_record.normal, self.index_of_refraction, cosine)
        } else {
            let cosine = -ray_in.direction().dot(&hit_record.normal) / ray_in.direction().magnitude();
            (hit_record.normal, 1.0 / self.index_of_refraction, cosine)
        };
        if let Some(refracted) = refract(&ray_in.direction(), &outward_normal, etai_over_etat) {
            let reflect_prob = reflectance(cosine, self.index_of_refraction);
            if rand::thread_rng().gen::<f64>() >= reflect_prob {
                let scattered = Ray::new(hit_record.p, refracted);
                return Some((scattered, attenuation))
            }
        }
        let reflected = reflect(ray_in.direction(), hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        Some((scattered, attenuation))
    }
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    return v - 2.0 * v.dot(&n) * n;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 -r0) * (1.0 - cosine).powi(5)
}

fn refract(v: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - etai_over_etat.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = etai_over_etat * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}