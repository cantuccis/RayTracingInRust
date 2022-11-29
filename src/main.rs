mod models;
use crate::models::camera::Camera;
use crate::models::color::Color;
use crate::models::hittable_list::HittableList;
use crate::models::material::{self, Dielectric, Lambertian, Metal};
use crate::models::point3::Point3;
use crate::models::sphere::Sphere;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;
use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;
use std::f64;
use std::mem::size_of;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    const N: usize = 1_000_000;

    std::thread::Builder::new()
        .stack_size(size_of::<f64>() * N)
        .spawn(|| {
            let result = set_up_logger();
            if result.is_err() {
                panic!()
            }

            // ZAWAAARDO (world)
            let mut world = HittableList::default();
            let material_ground = Arc::new(Lambertian {
                albedo: Color::new(0.5, 0.5, 0.5),
            });
            world.push(Sphere::new(
                Point3::new(0.0, -1000.0, 0.0),
                1000.0,
                material_ground.clone(),
            ));
            populate_random_scene(&mut world);

            //big balls
            let material1 = Arc::new(Dielectric {
                index_of_refraction: 1.5,
            });
            world.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

            let material2 = Arc::new(Lambertian {
                albedo: Color::new(0.4, 0.2, 0.1),
            });
            world.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

            let material3 = Arc::new(Metal {
                albedo: Color::new(0.7, 0.6, 0.5),
                fuzz: 0.0,
            });
            world.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

            // Image
            let aspect_ratio = 3.0 / 2.0;
            let image_width = 1200;
            let image_height = (image_width as f64 / aspect_ratio) as i32;
            let samples_per_pixel = 500;
            let max_depth = 50;
            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let vup = Vector3::new(0.0, 1.0, 0.0);
            let vfov = 20.0;
            let aperture = 0.1;
            let dist_to_focus = 10.0;
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
            // for j in (0..image_height).rev() {
            //     log::info!("Remaining lines: {j}");
            //     for i in 0..image_width {
            //         let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            //         for s in 0..samples_per_pixel {
            //             let u = (i as f64 + random_double()) / ((image_width - 1) as f64);
            //             let v = (j as f64 + random_double()) / ((image_height - 1) as f64);
            //             let r = cam.get_ray(u, v);
            //             pixel_color += r.color(&world, max_depth);
            //         }
            //         write_color(&pixel_color, samples_per_pixel);
            //     }
            // }
            let image_height_vec: Vec<i32> = (0..image_height).rev().collect::<Vec<_>>();
            for chunk in image_height_vec.chunks(image_height as usize) {
                log::info!("CHUNK START");
                let image = chunk
                    .into_par_iter()
                    .flat_map(|y| {
                        let collected = (0..image_width)
                            .flat_map(|x| {
                                let col: Vector3<f64> = (0..samples_per_pixel)
                                    .map(|_| {
                                        let u = (x as f64 + random_double())
                                            / ((image_width - 1) as f64);
                                        let v = (*y as f64 + random_double())
                                            / ((image_height - 1) as f64);
                                        let ray = cam.get_ray(u, v);
                                        ray.color(&world, max_depth)
                                    })
                                    .sum();
                                let collected_x = col
                                    .iter()
                                    .map(|c| {
                                        (255.99
                                            * (c / samples_per_pixel as f64)
                                                .sqrt()
                                                .max(0.0)
                                                .min(1.0))
                                            as u8
                                    })
                                    .collect::<Vec<u8>>();
                                collected_x
                            })
                            .collect::<Vec<u8>>();
                        log::info!("LINE");
                        collected
                    })
                    .collect::<Vec<u8>>();
                log::info!("CHUNK END");
                for col in image.chunks(3) {
                    println!("{} {} {}", col[0], col[1], col[2]);
                }
            }

            log::info!("Done.");
        })
        .unwrap()
        .join()
        .unwrap();
}

fn populate_random_scene(world: &mut HittableList) {
    for a in -11..11 {
        let coord_a = a as f64;
        for b in -11..11 {
            let coord_b = b as f64;
            let choose_material = random_double();
            let center = Point3::new(
                coord_a + 0.9 * random_double(),
                0.2,
                coord_b + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    let rv = random_vector();
                    let albedo = Color::new(rv.x * rv.x, rv.y * rv.y, rv.z * rv.z);
                    let sphere_material = Arc::new(Lambertian { albedo });
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else if choose_material < 0.95 {
                    let albedo = random_vector_within(0.5, 1.0);
                    let fuzz = random_double_within(0.0, 0.5);
                    let sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.push(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Arc::new(Dielectric {
                        index_of_refraction: 1.5,
                    });
                    world.push(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }
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
