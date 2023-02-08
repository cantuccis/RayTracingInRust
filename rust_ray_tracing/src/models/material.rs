use nalgebra::Vector3;

use super::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    util::{random_double, random_vector_in_unit_sphere},
};

pub trait Material: Send + Sync {
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
        let fuzz = if self.fuzz < 1.0 { self.fuzz } else { 1.0 };

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
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray_in.direction().normalize();

        let dot_product = (-unit_direction).dot(&hit_record.normal);
        let cos_theta = if dot_product < 1.0 { dot_product } else { 1.0 };
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(unit_direction, hit_record.normal)
            } else {
                refract(&unit_direction, &hit_record.normal, refraction_ratio)
            };
        let scattered = Ray::new(hit_record.p, direction);
        Some((scattered, attenuation))
    }
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    return v - 2.0 * v.dot(&n) * n;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(uv: &Vector3<f64>, n: &Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
    let dot_prod = (-uv).dot(&n);
    let cos_theta = if dot_prod < 1.0 { dot_prod } else { 1.0 };
    let r_out_prep = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_prep.magnitude_squared()).abs()).sqrt() * n;
    r_out_prep + r_out_parallel
    // let uv = v.normalize();
    // let dt = uv.dot(&n);
    // let discriminant = 1.0 - etai_over_etat.powi(2) * (1.0 - dt.powi(2));
    // if discriminant > 0.0 {
    //     let refracted = etai_over_etat * (uv - n * dt) - n * discriminant.sqrt();
    //     Some(refracted)
    // } else {
    //     None
    // }
}
