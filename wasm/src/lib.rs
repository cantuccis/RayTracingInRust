
use wasm_bindgen::prelude::*;
use rust_ray_tracing::models::renderer::Renderer;

#[wasm_bindgen]
pub fn render(image_width: i32, samples_per_pixel:i32, max_depth:i32) -> String {
    let renderer = Renderer::sample(image_width, samples_per_pixel, max_depth);
    let image_height = renderer.image_height;
    let mut final_image: Vec<u8> = vec![]; // Render
    let mut image_string = format!("P3\n{image_width} {image_height}\n255\n");

    let image_height_vec: Vec<i32> = (0..image_height).rev().collect::<Vec<_>>();
    for chunk in image_height_vec.chunks(image_height as usize) {
        let mut image = chunk
            .into_iter()
            .flat_map(|y| {
                renderer.render_line(*y)
            })
            .collect::<Vec<u8>>();
        final_image.append(&mut image);
    }
    for col in final_image.chunks(3) {
        image_string += format!("{} {} {}\n", col[0], col[1], col[2]).as_str();
    }
    return image_string;
}

// pub fn render(image_width: i32, samples_per_pixel:i32, max_depth:i32) -> String {    const N: usize = 1_000_000;

//     // ZAWAAARDO (world)
//     let mut world = HittableList::default();
//     let material_ground = Arc::new(Lambertian {
//         albedo: Color::new(0.5, 0.5, 0.5),
//     });
//     world.push(Sphere::new(
//         Point::new(0.0, -1000.0, 0.0),
//         1000.0,
//         material_ground.clone(),
//     ));
//     populate_random_scene(&mut world);

//     //big balls
//     let material1 = Arc::new(Dielectric {
//         index_of_refraction: 1.5,
//     });
//     world.push(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1));

//     let material2 = Arc::new(Lambertian {
//         albedo: Color::new(0.4, 0.2, 0.1),
//     });
//     world.push(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2));

//     let material3 = Arc::new(Metal {
//         albedo: Color::new(0.7, 0.6, 0.5),
//         fuzz: 0.0,
//     });
//     world.push(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3));

//     // Image
//     let aspect_ratio = 3.0 / 2.0;
//     // let image_width = 400;
//     let image_height = (image_width as f64 / aspect_ratio) as i32;
//     // let samples_per_pixel = 50;
//     // let max_depth = 50;
//     // Camera
//     let look_from = Point::new(13.0, 2.0, 3.0);
//     let look_at = Point::new(0.0, 0.0, 0.0);
//     let vup = Vector3::new(0.0, 1.0, 0.0);
//     let vfov = 20.0;
//     let aperture = 0.1;
//     let dist_to_focus = 10.0;
//     let cam = Camera::new(
//         look_from,
//         look_at,
//         vup,
//         vfov,
//         aspect_ratio,
//         aperture,
//         dist_to_focus,
//     );

//     let mut final_image: Vec<u8> = vec![]; // Render
//     let mut image_string = format!("P3\n{image_width} {image_height}\n255\n");
//     print!("P3\n{image_width} {image_height}\n255\n");

//     let image_height_vec: Vec<i32> = (0..image_height).rev().collect::<Vec<_>>();
//     for chunk in image_height_vec.chunks(image_height as usize) {
//         let mut image = chunk
//             .into_iter()
//             .flat_map(|y| {
//                 let collected = (0..image_width)
//                     .flat_map(|x| {
//                         let col: Vector3<f64> = (0..samples_per_pixel)
//                             .map(|_| {
//                                 let u = (x as f64 + random_double()) / ((image_width - 1) as f64);
//                                 let v = (*y as f64 + random_double()) / ((image_height - 1) as f64);
//                                 let ray = cam.get_ray(u, v);
//                                 ray.color(&world, max_depth)
//                             })
//                             .sum();
//                         let collected_x = col
//                             .iter()
//                             .map(|c| {
//                                 (255.99 * (c / samples_per_pixel as f64).sqrt().max(0.0).min(1.0))
//                                     as u8
//                             })
//                             .collect::<Vec<u8>>();
//                         collected_x
//                     })
//                     .collect::<Vec<u8>>();
//                 collected
//             })
//             .collect::<Vec<u8>>();
//         for col in image.chunks(3) {
//             println!("{} {} {}", col[0], col[1], col[2]);
//         }
//         final_image.append(&mut image);
//     }
//     for col in final_image.chunks(3) {
//         image_string += format!("{} {} {}\n", col[0], col[1], col[2]).as_str();
//     }
//     return image_string;
// }

