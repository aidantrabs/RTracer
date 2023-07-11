use crate::hittable::{Hittable, HitRecord};
use crate::Ray;

pub struct HittableList {
     objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
     pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
          return HittableList { objects: list };
     }

     pub fn add(&mut self, object: Box<dyn Hittable>) {
          self.objects.push(object);
     }
}

impl Hittable for HittableList {
     fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
          let mut hit_anything = None;
          let mut closest_so_far = t_max;

          for object in &self.objects {
               if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                    closest_so_far = hit_record.t;
                    hit_anything = Some(hit_record);
               }
          }

          return hit_anything;
     }
}