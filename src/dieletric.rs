use std::sync::Arc;
use rand::Rng;
use crate::Vec3;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::hittable::HitRecord;
use crate::material::Material;

pub struct Dielectric {
     refraction_index: f32,
}

impl Dielectric {
     pub fn new(ir: f32) -> Dielectric {
          return Dielectric { refraction_index: ir };
     }

     fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
          let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
          r0 = r0 * r0;

          return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
     }
}

impl Material for Dielectric {
     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
          let refraction_ratio: f32 = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
          let unit_direction = Vec3::unit_vector(r_in.direction());

          let cos_theta = Vec3::dot(-1.0 * unit_direction, rec.normal).min(1.0);
          let sin_theta = ((1.0 - cos_theta.powi(2)) as f32).sqrt();

          let mut rng = rand::thread_rng();
          let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
          let can_reflect = rng.gen::<f32>() < Dielectric::reflectance(cos_theta, refraction_ratio);

          let direction: Vec3 = if cannot_refract || can_reflect {
               Vec3::reflect(&unit_direction, &rec.normal)
          } else {
               unit_direction.refract(rec.normal, refraction_ratio)
          };

          let scattered = Ray::new(rec.p, direction);

          return Some((Color::new(1.0, 1.0, 1.0), scattered));
     }

     fn clone(&self) -> Arc<dyn Material> {
          return Arc::new(Dielectric::new(self.refraction_index.clone()));
     }
}