use super::camera::Camera;
use super::color::Color;
use super::hittable::Hittable;
use super::hittable_list::HittableList;
use super::material::*;
use super::point::Point;
use super::sphere::Sphere;
use super::util::*;
use nalgebra::Vector3;
use std::sync::Arc;

pub struct Renderer {
    pub image_width: i32,
    pub image_height: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub cam: Camera,
    pub world: Arc<dyn Hittable>,
}

impl Renderer {
    pub fn render_line(&self, line_number: i32) -> Vec<u8> {
        let collected = (0..self.image_width)
            .flat_map(|x| {
                let sampled_pixel: Color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let u = (x as f64 + random_double()) / ((self.image_width - 1) as f64);
                        let v = (line_number as f64 + random_double())
                            / ((self.image_height - 1) as f64);
                        let ray = self.cam.get_ray(u, v);
                        ray.color(self.world.as_ref(), self.max_depth)
                    })
                    .sum();
                self.pixel_to_rgb(&sampled_pixel)
            })
            .collect::<Vec<u8>>();
        collected
    }

    pub fn render_pixel(&self, x: i32, y: i32) -> Vec<u8> {
        let sampled_pixel: Color = (0..self.samples_per_pixel)
            .map(|_| {
                let u = (x as f64 + random_double()) / ((self.image_width - 1) as f64);
                let v = (y as f64 + random_double()) / ((self.image_height - 1) as f64);
                let ray = self.cam.get_ray(u, v);
                ray.color(self.world.as_ref(), self.max_depth)
            })
            .sum();
        self.pixel_to_rgb(&sampled_pixel)
    }

    fn pixel_to_rgb(&self, pixel_color: &Color) -> Vec<u8> {
        let scale = 1.0 / (self.samples_per_pixel as f64);
        let r = (pixel_color.x * scale).sqrt();
        let g = (pixel_color.y * scale).sqrt();
        let b = (pixel_color.z * scale).sqrt();
        let clamped_r = (256.0 * clamp(r, 0.0, 0.999)) as u8;
        let clamped_g = (256.0 * clamp(g, 0.0, 0.999)) as u8;
        let clamped_b = (256.0 * clamp(b, 0.0, 0.999)) as u8;
        vec![clamped_r, clamped_g, clamped_b]
    }

    pub fn sample(image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        // ZAWAAARDO (world)
        let mut world = HittableList::default();

        // Earth
        let material_ground = Arc::new(Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        });
        world.push(Sphere::new(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground.clone(),
        ));

        // Random little spheres
        Self::populate_random_scene(&mut world);

        // Big spheres
        let material1 = Arc::new(Dielectric {
            index_of_refraction: 1.5,
        });
        let material2 = Arc::new(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        });
        let material3 = Arc::new(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        });
        world.push(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1));
        world.push(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2));
        world.push(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3));

        // Image
        let aspect_ratio = 3.0 / 2.0;
        let image_height = (image_width as f64 / aspect_ratio) as i32;

        // Camera
        let look_from = Point::new(13.0, 2.0, 3.0);
        let look_at = Point::new(0.0, 0.0, 0.0);
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

        Renderer {
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
            cam,
            world: Arc::new(world),
        }
    }

    fn populate_random_scene(world: &mut HittableList) {
        for a in 0..6 {
            let coord_a = a as f64;
            for b in -1..3 {
                let coord_b = b as f64;
                let choose_material = random_double();
                let center = Point::new(
                    coord_a + 0.9 * random_double(),
                    0.2,
                    coord_b + 0.9 * random_double(),
                );

                if (center - Point::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
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
}
