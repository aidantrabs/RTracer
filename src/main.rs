mod vec3;
mod ray;
mod hittable;
mod utils;
mod sphere;
mod hittable_list;
mod camera;
mod lambertian;
mod material;
mod metal;
mod dieletric;

use std;
use std::sync::Arc;
use rayon::prelude::*;
use rand::Rng;
use sphere::Sphere;
use vec3::Vec3;
use vec3::Color;
use vec3::Point3;
use ray::Ray;
use hittable_list::HittableList;
use camera::Camera;
use lambertian::Lambertian;
use utils::{write_color, ray_color};
use crate::hittable::Hittable;
use crate::dieletric::Dielectric;
use crate::metal::Metal;

/*
     @Description: Handles the creation of the world
     @Params: None
     @Returns: None
*/
fn main() {
     const IMAGE_WIDTH: i32 = 1200;
     const ASPECT_RATIO: f32 = 3.0 / 2.0;
     const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
     const SAMPLES_PER_PIXEL: i32 = 500;
     const MAX_DEPTH: i32 = 50;

     let world = random_scene();

     let from = Point3::new(13.0, 2.0, 3.0);
     let at = Point3::new(0.0, 0.0, 0.0);
     let vup = Vec3::new(0.0, 1.0, 0.0);
     let dist_to_focus = 10.0;
     let aperture = 0.1;
     let cam: Camera = Camera::new(from, at, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

     print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
     for j in (0..IMAGE_HEIGHT).rev() {
          eprintln!("Scanlines remaining: {}", j + 1);

          let scanline: Vec<Color> = (0..IMAGE_WIDTH).into_par_iter().map(|i| {
               let mut pixel_color = Color::new(0.0, 0.0, 0.0);
               for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let r = cam.get_ray(u as f32, v as f32);
                    pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
               }

               pixel_color
          }).collect();

          for pixel_color in scanline {
               write_color(pixel_color, SAMPLES_PER_PIXEL);
          }
     }

     eprintln!("\nCompleted!\n");
}

/*
     @Description: Generates a random scene
     @Params: None
     @Returns: HittableList
*/
fn random_scene() -> HittableList {
     let mut rng = rand::thread_rng();
     let world_list: Vec<Box<dyn Hittable>> = Vec::new();
     let mut world = HittableList::new(world_list);

     let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
     let sphere_ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground);
     world.add(Box::new(sphere_ground));

     for i in -11..=11 {
          for j in -11..=11 {
               let choose_material = rng.gen::<f32>();
               let center = Point3::new((i as f32) + rng.gen_range(0.0..0.9), 0.2, (j as f32) + rng.gen_range(0.0..0.9));
               
               if choose_material < 0.8 {
                    let albedo = Color::random_range(0.0, 1.0) * Color::random_range(0.0, 1.0);
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let sphere = Sphere::new(center, 0.2, sphere_material);

                    world.add(Box::new(sphere));
               }

               else if choose_material < 0.95 {
                    let albedo = Color::random_range(0.4, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere::new(center, 0.2, sphere_material);
                    world.add(Box::new(sphere));
               }

               else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere::new(center, 0.2, sphere_material);
                    world.add(Box::new(sphere));
               }
          }
     }

     let mat1 = Arc::new(Dielectric::new(1.5));
     let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
     let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

     let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
     let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
     let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

     world.add(Box::new(sphere1));
     world.add(Box::new(sphere2));
     world.add(Box::new(sphere3));

     return world;
}