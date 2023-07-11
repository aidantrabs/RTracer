use std::sync::Arc;
use crate::ray::Ray;
use crate::material::Material;
use crate::hittable::HitRecord;
use crate::{Color, Vec3};

pub struct Lambertian {
     pub(crate) albedo: Color,
}

impl Lambertian {
     pub fn new(a: Color) -> Lambertian {
          return Lambertian {
               albedo: a,
          }
     }
}

impl Material for Lambertian {
     fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
          let mut scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();

          if scatter_direction.near_zero() {
               scatter_direction = rec.normal;
          }

          let scattered = Ray::new(rec.p, scatter_direction);
          return Some((self.albedo, scattered));
     }

     fn clone(&self) -> Arc<dyn Material> {
          return Arc::new(Lambertian {
               albedo: self.albedo,
          });
     }
}