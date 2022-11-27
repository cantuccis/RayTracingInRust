mod models;
use rand::Rng;
use crate::models::camera::Camera;
use crate::models::color::Color;
use crate::models::hittable_list::HittableList;
use crate::models::point3::Point3;
use crate::models::ray::Ray;
use crate::models::sphere::Sphere;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::init_config;
use nalgebra::Vector3;

fn main() {
    let result = set_up_logger();
    if result.is_err() {
        panic!()
    }

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // ZAWAAARDO
    let sphere3 = Sphere::new(Point3::new(2.0, 2.0, -3.0), 0.8);
    let sphere1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    
    let mut world = HittableList::default();
    world.push(sphere1);
    world.push(sphere3);
    world.push(sphere2);


    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let cam = Camera::new(viewport_height, viewport_width, focal_length);
    
    // Render
    print!("P3\n {image_width} {image_height} \n255\n");
    log::info!("Start rendering");
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0,0.0,0.0); 
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / ((image_width - 1) as f64);
                let v = (j as f64 + random_double()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += r.color(&world);
            }
            write_color(&pixel_color, samples_per_pixel);
        }
    }
    log::info!("Done.");
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

fn write_color(pixel_color: &Color, samples_per_pixel:i32) {
    let scale = 1.0 / (samples_per_pixel as f64);
    let r = pixel_color.x * scale;
    let g = pixel_color.y * scale;
    let b = pixel_color.z * scale;
    let clamped_r = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let clamped_g = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let clamped_b = (256.0 * clamp(b, 0.0, 0.999)) as i32;
    print!("{clamped_r} {clamped_g} {clamped_b} \n");
}


fn random_double() -> f64{
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0);
}

fn random_double_within(min:f64, max:f64) -> f64{
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

fn clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}