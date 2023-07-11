use crate::Vec3;
use crate::vec3::Point3;
/*
     @Description: A struct that holds information about a ray
     @Params: origin: Point3, direction: Vec3
     @Returns: None
*/
#[derive(Debug, Copy, Clone)]
pub struct Ray {
     pub origin: Point3,
     pub direction: Vec3,
}

/*
     @Description: Implementation of the Ray struct
     @Function: new - Creates a new ray
     @Function: origin - Returns the origin of the ray
     @Function: direction - Returns the direction of the ray
     @Function: at - Returns the point at a given time
     @Returns: None
*/
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