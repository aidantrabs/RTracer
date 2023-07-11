use std::sync::Arc;
use crate::ray::Ray;
use crate::material::Material;
use crate::hittable::HitRecord;
use crate::{Color, Vec3};

/*
     @Description: A struct that holds information about a metal material
     @Params: albedo: Color, fuzz: f32
     @Returns: None
*/
pub struct Metal {
     pub albedo: Color,
     pub fuzz: f32,
}

/*
     @Description: Implementation of the Metal struct
     @Function: new - Creates a new Metal material
     @Returns: Metal
*/
impl Metal {
     pub fn new(a: Color, f: f32) -> Metal {
          let mut fz = f;

          if f >= 1.0 {
               fz = 1.0;
          }

          return Metal { albedo: a, fuzz: fz }
     }
}

/*
     @Description: Implementation of the Material trait for Metal
     @Function: scatter - Determines if a ray is scattered or not
     @Function: clone - Clones the material
     @Returns: Metal Material
*/
impl Material for Metal {
     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
          let reflected: Vec3 = Vec3::reflect(&(Vec3::unit_vector(r_in.direction())), &rec.normal);
          let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

          if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
               return Some((self.albedo, scattered));
          }

          else {
               return None;
          }
     }

     fn clone(&self) -> Arc<dyn Material> {
          return Arc::new(Metal { albedo: self.albedo, fuzz: self.fuzz });
     }
}