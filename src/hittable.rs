use std::sync::Arc;
use crate::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::material::Material;

/*
     @Description: A struct that holds information about a hit record
     @Params: p: Point3, normal: Vec3, material_ptr: Arc<dyn Material>, t: f32, front_face: bool
     @Returns: None
*/
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material_ptr: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

/*
     @Description: Implementation of the HitRecord struct
     @Function: set_face_normal - Sets the normal of the hit record
     @Returns: None
*/
impl HitRecord {
     #[inline]
     pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
          self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.0;
          self.normal = if self.front_face { *outward_normal } else { -1.0 * (*outward_normal) };
     }
}

/*
     @Description: A trait that defines the methods that a hittable object must implement
     @Function: hit - Determines if a ray hits an object
     @Returns: None
*/
pub trait Hittable: Send + Sync {
     fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}