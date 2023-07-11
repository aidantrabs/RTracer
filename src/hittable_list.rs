use crate::hittable::{Hittable, HitRecord};
use crate::Ray;

/*
     @Description: A list of hittable objects
     @Params: objects: Vec<Box<dyn Hittable>>
     @Returns: None
*/
pub struct HittableList {
     objects: Vec<Box<dyn Hittable>>,
}

/*
     @Description: Implementation of HittableList
     @Function: new - Creates a new HittableList
     @Function: add - Adds an object to the list
     @Returns: HittableList
*/
impl HittableList {
     pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
          return HittableList { objects: list };
     }

     pub fn add(&mut self, object: Box<dyn Hittable>) {
          self.objects.push(object);
     }
}

/*
     @Description: Implementation of Hittable for HittableList
     @Function: hit - Determines if a ray hits an object in the list
     @Returns: HittableList
*/
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