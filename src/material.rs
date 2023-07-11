use std::sync::Arc;
use crate::ray::Ray;
use crate::Color;
use crate::hittable::HitRecord;

/*
     @Description: A trait that defines the methods that a material must implement
     @Function: scatter - Determines if a ray is scattered or not
     @Function: clone - Clones the material
     @Returns: None
*/
pub trait Material: Send + Sync {
     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
     fn clone(&self) -> Arc<dyn Material>;
}