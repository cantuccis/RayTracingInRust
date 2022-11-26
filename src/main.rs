mod models;

use crate::models::color::Color;
use crate::models::point3::Point3;
use crate::models::ray::Ray;
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
    let image_height = ((image_width as f64) / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n {image_width} {image_height} \n255\n");

    for j in 0..image_height {
        log::info!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            write_color(&r.color());
        }
    }
    log::info!("\nDone.\n");
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

fn write_color(pixel_color: &Color) {
    let ir = (255.999 * pixel_color.x) as i32;
    let ig = (255.999 * pixel_color.y) as i32;
    let ib = (255.999 * pixel_color.z) as i32;
    print!("{ir} {ig} {ib} \n");
}
