use std::sync::Arc;
use crate::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material_ptr: Arc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
     #[inline]
     pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
          self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.0;
          self.normal = if self.front_face { *outward_normal } else { -1.0 * (*outward_normal) };
     }
}

pub trait Hittable: Send + Sync {
     fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}