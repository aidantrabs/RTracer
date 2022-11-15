use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
     e: [f32; 3],
}

impl Vec3 {
     pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
          Vec3 {
               e: [e0, e1, e2]
          }
     }

     pub fn x(&self) -> f32 {
          self.e[0]
     }

     pub fn y(&self) -> f32 {
          self.e[1]
     }

     pub fn z(&self) -> f32 {
          self.e[2]
     }
     
     pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
          u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
     }

     pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
          Vec3 {
              e: [
                    u.e[1] * v.e[2] - u.e[2] * v.e[1],
                    u.e[2] * v.e[0] - u.e[0] * v.e[2],
                    u.e[0] * v.e[1] - u.e[1] * v.e[0],
              ] 
          }
     }

     pub fn length(self) -> f32 {
          (self.e[0] * self.e[0] + self.e[1] * self.e[1]+ self.e[2] * self.e[2]).sqrt()
     }

     pub fn length_squared(self) -> f32 {
          self.e[0] * self.e[0] + self.e[1] * self.e[1]+ self.e[2] * self.e[2]
     }

     pub fn unit_vector(u: &Vec3) -> Vec3 {
          *u / u.length()
     }

     pub fn random_in_unit_sphere() -> Vec3 {
          loop {
               let p = Vec3::new(
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
               ) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
               if p.length_squared() < 1.0 {
                    return p;
               }
          }
     }
}

impl Add for Vec3 {
     type Output = Self;

     fn add(self, v: Vec3) -> Vec3 {
          Vec3 {
               e: [
                    self.e[0] + v.e[0],
                    self.e[1] + v.e[1],
                    self.e[2] + v.e[2],
               ],
          }
     }
}

impl Sub for Vec3 {
     type Output = Self;

     fn sub(self, v: Vec3) -> Vec3 {
          Vec3 {
               e: [
                    self.e[0] - v.e[0],
                    self.e[1] - v.e[1],
                    self.e[2] - v.e[2],
               ],
          }
     }
}

impl Mul for Vec3 {
     type Output = Vec3;

     fn mul(self, v: Vec3) -> Vec3 {
          Vec3 {
               e: [
                    self.e[0] * v.e[0],
                    self.e[1] * v.e[1],
                    self.e[2] * v.e[2],
               ],
          }
     }
}

impl Mul<f32> for Vec3 {
     type Output = Vec3;

     fn mul(self, v: f32) -> Vec3 {
          Vec3 {
               e: [
                    v * self.e[0],
                    v * self.e[1],
                    v * self.e[2],
               ],
          }
     }
}

impl Mul<Vec3> for f32 {
     type Output = Vec3;

     fn mul(self, v: Vec3) -> Vec3 {
          Vec3 {
               e: [
                    self * v.e[0],
                    self * v.e[1],
                    self * v.e[2],
               ],
          }
     }
}

impl Div<f32> for Vec3 {
     type Output = Vec3;

     fn div(self, v: f32) -> Vec3 {
          let k = 1.0 / v; 

          Vec3 {
               e: [
                    self.e[0] * k,
                    self.e[1] * k,
                    self.e[2] * k,
               ],
          }
     }
}

impl Div<Vec3> for f32 {
     type Output = Vec3;

     fn div(self, v: Vec3) -> Vec3 {
          let k = 1.0 / self; 

          Vec3 {
               e: [
                    v.e[0] * k,
                    v.e[1] * k,
                    v.e[2] * k,
               ],
          }
     }
}

impl Neg for Vec3 {
     type Output = Vec3;

     fn neg(self) -> Vec3 {
          Vec3 {
               e: [
                    -self.e[0],
                    -self.e[1],
                    -self.e[2],
               ],
          }
     }
}



