use std::env;

use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use rust_ray_tracing::models::renderer::Renderer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rust_ray_tracing_cli <image-width>");
        return;
    }
    let image_width = if let Ok(image_width) = args[1].parse::<i32>() {
        image_width
    } else {
        panic!("Invalid image width. Please input a valid number")
    };

    let samples_per_pixel = if args.len() >= 3 {
        if let Ok(samples_per_pixel) = args[2].parse::<i32>() {
            samples_per_pixel
        } else {
            panic!("Invalid samples per pixel. Please input a valid number")
        }
    } else {
        200
    };

    let renderer = Renderer::sample(image_width, samples_per_pixel, 25);
    let height = renderer.image_height;
    let width = renderer.image_width;

    // Allocate the pixel data which our threads will be writing into.
    let pixels = (width * height) as usize;
    let mut rgb_data = vec![0; 3 * pixels];

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(12)
        .build()
        .unwrap();

    thread_pool.install(|| {
        rgb_data
            .par_chunks_mut(3)
            .enumerate()
            .for_each(|(i, chunk)| {
                let i = i as i32;
                let x = i % width;
                let y = i / width;
                let pixel = renderer.render_pixel(x, height - y);
                chunk[0] = pixel[0];
                chunk[1] = pixel[1];
                chunk[2] = pixel[2];
            });
    });

    print!("P3\n{width} {height}\n255\n");
    for col in rgb_data.chunks(3) {
        println!("{} {} {}", col[0], col[1], col[2]);
    }
}
