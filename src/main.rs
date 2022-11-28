mod models;
use std::f64;
use std::mem::size_of;
use std::rc::Rc;

use crate::models::camera::Camera;
use crate::models::color::Color;
use crate::models::hittable_list::HittableList;
use crate::models::material::{Dielectric, Lambertian, Metal};
use crate::models::point3::Point3;
use crate::models::sphere::Sphere;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;
use nalgebra::Vector3;
use rand::Rng;

fn main() {
    const N: usize = 1_000_000;

    std::thread::Builder::new()
        .stack_size(size_of::<f64>() * N)
        .spawn(|| {
            let result = set_up_logger();
            if result.is_err() {
                panic!()
            }

            // Image
            let aspect_ratio = 16.0 / 9.0;
            let image_width = 400;
            let image_height = (image_width as f64 / aspect_ratio) as i32;
            let samples_per_pixel = 100;
            let max_depth = 50;

            // ZAWAAARDO (world)
            // let R = f64::cos(f64::consts::PI / 4.0);
            let material_ground = Rc::new(Lambertian {
                albedo: Color::new(0.8, 0.8, 0.0),
            });
            let material_center = Rc::new(Lambertian {
                albedo: Color::new(0.1, 0.2, 0.5),
            });
            let material_left = Rc::new(Dielectric {
                index_of_refraction: 1.5,
            });
            let material_right = Rc::new(Metal {
                albedo: Color::new(0.8, 0.6, 0.2),
                fuzz: 0.0,
            });
            // let material_left = Lambertian {
            //     albedo: Color::new(0.0,0.0,1.0)
            // };
            // let material_right = Lambertian {
            //     albedo: Color::new(1.0,0.0,0.0)
            // };

            let mut world = HittableList::default();
            world.push(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground.clone(),
            ));
            world.push(Sphere::new(
                Point3::new(0.0, 0.0, -1.0),
                0.5,
                material_center.clone(),
            ));
            world.push(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                0.5,
                material_left.clone(),
            ));
            world.push(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                -0.45,
                material_left.clone(),
            ));
            world.push(Sphere::new(
                Point3::new(1.0, 0.0, -1.0),
                0.5,
                material_right.clone(),
            ));
            // world.push(Sphere::new(
            //     Point3::new(-R, 0.0, -1.0),
            //     R,
            //     material_left,
            // ));
            // world.push(Sphere::new(
            //     Point3::new(R, 0.0, -1.0),
            //     R,
            //     material_right,
            // ));

            // Camera
            let look_from = Point3::new(3.0, 3.0, 2.0);
            let look_at = Point3::new(0.0, 0.0, -1.0);
            let vup = Vector3::new(0.0, 1.0, 0.0);
            let vfov = 20.0;
            let aperture = 2.0;
            let dist_to_focus = (look_from - look_at).magnitude() as f64;
            let cam = Camera::new(
                look_from,
                look_at,
                vup,
                vfov,
                aspect_ratio,
                aperture,
                dist_to_focus,
            );

            // Render
            print!("P3\n {image_width} {image_height} \n255\n");
            log::info!("Start rendering");
            for j in (0..image_height).rev() {
                log::info!("Remaining lines: {j}");
                for i in 0..image_width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for s in 0..samples_per_pixel {
                        let u = (i as f64 + random_double()) / ((image_width - 1) as f64);
                        let v = (j as f64 + random_double()) / ((image_height - 1) as f64);
                        let r = cam.get_ray(u, v);
                        pixel_color += r.color(&world, max_depth);
                    }
                    write_color(&pixel_color, samples_per_pixel);
                }
            }
            log::info!("Done.");
        })
        .unwrap()
        .join()
        .unwrap();
}

fn set_up_logger() -> Result<(), RendererError> {
    let Ok(logfile) = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build("output.log") else { return Err(RendererError::ConfigurationNotLoaded) };

    let Ok(config) = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info)) 
        else { return Err(RendererError::ConfigurationNotLoaded) };

    match init_config(config) {
        Ok(it) => it,
        Err(_err) => return Err(RendererError::ConfigurationNotLoaded),
    };

    Ok(())
}

#[derive(Debug)]
pub enum RendererError {
    ConfigurationNotLoaded,
}

fn write_color(pixel_color: &Color, samples_per_pixel: i32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();
    let clamped_r = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let clamped_g = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let clamped_b = (256.0 * clamp(b, 0.0, 0.999)) as i32;
    print!("{clamped_r} {clamped_g} {clamped_b} \n");
}

fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0);
}

fn random_double_within(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn random_vector() -> Vector3<f64> {
    Vector3::new(random_double(), random_double(), random_double())
}

fn random_vector_within(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        random_double_within(min, max),
        random_double_within(min, max),
        random_double_within(min, max),
    )
}

fn random_vector_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p = random_vector_within(-1.0, 1.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

fn random_vector_in_unit_disk() -> Vector3<f64> {
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

fn random_in_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let in_unit_sphere = random_vector_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}
