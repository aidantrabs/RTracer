use std::sync::Arc;
use crate:ray::Ray;
use crate::Color;
use crate::hittable::HitRecord;

pub trait Material: Send + Sync {
     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
     fn clone(&self) -> Arc<dyn Material>;
}