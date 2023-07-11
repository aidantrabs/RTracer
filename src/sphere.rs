use std::sync::Arc;
use crate::Ray;
use crate::vec3::{Point3, Vec3};
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;

/*
     @Description: A struct that holds information about a sphere
     @Params: center: Point3, radius: f32, material_ptr: Arc<dyn Material>
     @Returns: None
*/
pub struct Sphere {
     pub center: Point3,
     pub radius: f32,
     material_ptr: Arc<dyn Material>,
}

/*
     @Description: Implementation of the Sphere struct
     @Function: new - Creates a new sphere
     @Returns: Sphere
*/
impl Sphere {
     pub fn new(cen: Point3, r: f32, m: Arc<dyn Material>) -> Sphere {
          Sphere {
               center: cen,
               radius: r,
               material_ptr: m,
          }
     }
}

/*
     @Description: Implementation of the Hittable trait for Sphere
     @Function: hit - Determines if a ray hits a sphere
     @Returns: Option<HitRecord>
*/
impl Hittable for Sphere {
     fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
          let oc = r.origin() - self.center;
          let a = r.direction().length_squared();
          let half_b = Vec3::dot(oc, r.direction());
          let c = oc.length_squared() - self.radius.powi(2);

          let discriminant = half_b.powi(2) - a * c;

          if discriminant < 0.0 {
               return None;
          }

          let sqrtd = discriminant.sqrt();

          let mut root = (-half_b - sqrtd) / a;

          if root < t_min || t_max < root {
               root = (-half_b + sqrtd) / a;

               if root < t_min || t_max < root {
                    return None;
               }
          }

          let mut rec = HitRecord {
               t: root,
               p: r.at(root),
               normal: Vec3::new(0.0, 0.0, 0.0),
               material_ptr: self.material_ptr.clone(),
               front_face: false,
          };

          let outward_normal = (rec.p - self.center) / self.radius;
          rec.set_face_normal(r, &outward_normal);

          return Some(rec);
     }
}