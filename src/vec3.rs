use std::ops::{Add, Sub, Mul, Div};
use crate::utils::{random_f32, random_f32_range};

/*
     @Description: A struct that holds information about a 3D vector
     @Params: None
     @Returns: None
*/
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
     e: [f32; 3],
}

/*
     @Description: Implementation of the Vec3 struct
     @Function: new - Creates a new Vec3
     @Function: x - Returns the x value of the Vec3
     @Function: y - Returns the y value of the Vec3
     @Function: z - Returns the z value of the Vec3
     @Function: length - Returns the length of the Vec3
     @Function: length_squared - Returns the length squared of the Vec3
     @Function: dot - Returns the dot product of two Vec3s
     @Function: cross - Returns the cross product of two Vec3s
     @Function: unit_vector - Returns the unit vector of a Vec3
     @Function: random - Returns a random Vec3
     @Function: random_range - Returns a random Vec3 in a given range
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_unit_vector - Returns a random unit vector
     @Function: random_in_hemisphere - Returns a random Vec3 in a hemisphere
     @Function: random_in_unit_disk - Returns a random Vec3 in a unit disk
     @Function: near_zero - Returns true if the Vec3 is near zero
     @Function: reflect - Returns the reflection of a Vec3
     @Function: refract - Returns the refraction of a Vec3
     @Function: reflectance - Returns the reflectance of a Vec3
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Function: random_in_unit_sphere - Returns a random Vec3 in a unit sphere
     @Returns: None
*/
impl Vec3 {
     pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
          let vec = Vec3 {
               e: [e0, e1, e2]
          };

          return vec;
     }

     pub fn x(&self) -> f32 {
          return self.e[0]
     }

     pub fn y(&self) -> f32 {
          return self.e[1]
     }

     pub fn z(&self) -> f32 {
          return self.e[2]
     }

     pub fn length(&self) -> f32 {
          return (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt() as f32;
     }

     pub fn length_squared(self) -> f32 {
          return self.length() * self.length();
     }
     
     #[inline]
     pub fn dot(self, v: Vec3) -> f32 {
          return self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2];
     }

     #[inline]
     pub fn cross(self, v: Vec3) -> Vec3 {
          return Vec3 {
               e: [
                    self.e[1] * v.e[2] - self.e[2] * v.e[1],
                    self.e[2] * v.e[0] - self.e[0] * v.e[2],
                    self.e[0] * v.e[1] - self.e[1] * v.e[0],
               ] 
          }
     }

     #[inline]
     pub fn unit_vector(v: Vec3) -> Vec3 {
          return v / v.length();
     }

     #[inline]
     pub fn random() -> Vec3 {
          return Vec3 {
               e: [
                    random_f32(),
                    random_f32(),
                    random_f32(),
               ]
          }
     }

     #[inline]
     pub fn random_range(min: f32, max: f32) -> Vec3 {
          return Vec3 {
               e: [
                    random_f32_range(min, max),
                    random_f32_range(min, max),
                    random_f32_range(min, max),
               ]
          }
     }

     #[inline]
     pub fn random_in_unit_sphere() -> Vec3 {
          loop {
               let p = Vec3::random_range(-1.0, 1.0);

               if p.length_squared() >= 1.0 {
                    continue;
               }

               return p;
          }
     }

     pub fn random_unit_vector() -> Vec3 {
          return Vec3::unit_vector(Vec3::random_in_unit_sphere());
     }

     pub fn random_in_unit_disk() -> Vec3 {
          loop {
               let p = Vec3::new(random_f32_range(-1.0, 1.0), random_f32_range(-1.0, 1.0), 0.0);

               if p.length_squared() >= 1.0 {
                    continue;
               }

               return p;
          }
     }

     pub fn near_zero(&self) -> bool {
          const S: f32 = (1 / 10_i32.pow(8)) as f32;

          return (self.e[0].abs() < S) && (self.e[1].abs() < S) && (self.e[2].abs() < S);
     }

     pub fn change(&mut self, v: Vec3) {
          self.e[0] = v.x();
          self.e[1] = v.y();
          self.e[2] = v.z();
     }

     pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
          return (*v) - 2.0 * v.dot(*n) * (*n);
     }

     pub fn refract(self, n: Vec3, etai_over_etat: f32) -> Vec3 {
          let cos_theta = Vec3::dot(-1.0 * self, n).min(1.0);
          let r_out_parallel = etai_over_etat * (self + cos_theta * n);
          let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;

          return r_out_parallel + r_out_perp;
     }
}

impl Add for Vec3 {
     type Output = Vec3;

     #[inline]
     fn add(self, _v: Vec3) -> Self::Output {
          return Vec3 {
               e: [
                    self.e[0] + _v.e[0],
                    self.e[1] + _v.e[1],
                    self.e[2] + _v.e[2],
               ],
          }
     }
}

impl Sub for Vec3 {
     type Output = Vec3;

     #[inline]
     fn sub(self, _v: Vec3) -> Self::Output {
          return Vec3 {
               e: [
                    self.e[0] - _v.e[0],
                    self.e[1] - _v.e[1],
                    self.e[2] - _v.e[2],
               ],
          }
     }
}

impl Mul for Vec3 {
     type Output = Self;

     #[inline]
     fn mul(self, _v: Vec3) -> Vec3 {
          Vec3 {
               e: [
                    self.e[0] * _v.e[0],
                    self.e[1] * _v.e[1],
                    self.e[2] * _v.e[2],
               ],
          }
     }
}

impl Mul<Vec3> for f32 {
     type Output = Vec3;

     #[inline]
     fn mul(self, mut _v: Vec3) -> Self::Output {
          _v.e[0] *= self;
          _v.e[1] *= self;
          _v.e[2] *= self;

          return _v;
     }
}

impl Div for Vec3 {
     type Output = Self;

     #[inline]
     fn div(self, _v: Vec3) -> Vec3 {
          return Vec3 {
               e: [
                    self.e[0] / _v.e[0],
                    self.e[1] / _v.e[1],
                    self.e[2] / _v.e[2],
               ],
          }
     }
}

impl Div<f32> for Vec3 {
     type Output = Self;

     fn div(self, _v: f32) -> Self::Output {
          return Vec3 {
               e: [
                    self.e[0] / _v,
                    self.e[1] / _v,
                    self.e[2] / _v,
               ],
          }
     }
}

pub type Point3 = Vec3;
pub type Color = Vec3;