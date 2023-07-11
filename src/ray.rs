use crate::Vec3;
use crate::vec3::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
     pub origin: Point3,
     pub direction: Vec3,
}

impl Ray {
     pub fn new(orig: Point3, dir: Vec3) -> Ray {
          return Ray {
               origin: orig,
               direction: dir,
          };
     }

     pub fn origin(&self) -> Point3 {
          return self.origin;
     }

     pub fn direction(&self) -> Point3 {
          return self.direction;
     }

     pub fn at(&self, t: f32) -> Point3 {
          return self.origin + t * self.direction
     }
}