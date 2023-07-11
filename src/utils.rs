use rand::Rng;
use crate::vec3::{Color, Vec3};
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;

pub static PI: f64 = 3.1415926535897932385;

#[inline]
pub fn random_f32() -> f32 {
     let mut rng = rand::thread_rng();
     return rng.gen_range(0.0..1.0) as f32;
}

#[inline]
pub fn random_f32_range(min: f32, max: f32) -> f32 {
     let mut rng = rand::thread_rng();
     return rng.gen_range(min..max) as f32
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
     if x < min {
          return min;
     }
     
     if x > max {
          return max;
     }

     return x;
}

pub fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
     if depth <= 0 {
          return Color::new(0.0, 0.0, 0.0);
     }

     if let Some(rec) = world.hit(r, 0.001, f64::INFINITY as f32) {
          if let Some((attenuation, scattered)) = rec.material_ptr.scatter(r, &rec) {
               attenuation * ray_color(&scattered, world, depth - 1)
          } else {
               Color::new(0.0, 0.0, 0.0)
          }
     } else {
          let unit_direction = Vec3::unit_vector(r.direction());
          let t = 0.5 * (unit_direction.y() + 1.0);
          (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
     }
}

pub fn write_color(v: Color, samples_per_pixel: i32) {
     let scale = 1.0 / (samples_per_pixel as f32);
     let r: f32 = (v.x() * scale).sqrt();
     let g: f32 = (v.y() * scale).sqrt();
     let b: f32 = (v.z() * scale).sqrt();

     println!(
          "{} {} {}", 
          (256.0 * clamp(r, 0.0, 0.999)) as i32, 
          (256.0 * clamp(g, 0.0, 0.999)) as i32, 
          (256.0 * clamp(b, 0.0, 0.999)) as i32
     );
}