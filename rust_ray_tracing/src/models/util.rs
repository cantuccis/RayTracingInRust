use nalgebra::Vector3;
use rand::Rng;


pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0);
}

pub fn random_double_within(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn random_vector() -> Vector3<f64> {
    Vector3::new(random_double(), random_double(), random_double())
}

pub fn random_vector_within(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        random_double_within(min, max),
        random_double_within(min, max),
        random_double_within(min, max),
    )
}

pub fn random_vector_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vector_within(-1.0, 1.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_vector_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = Vector3::new(
            random_double_within(-1.0, 1.0),
            random_double_within(-1.0, 1.0),
            0.0,
        );
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = random_vector_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}
