use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
     pub fn new(r: f32, g: f32, b: f32) -> Color {
          Color { r, g, b }
     }
     
     pub fn r(&self) -> f32 {
          self.r
     }
     
     pub fn g(&self) -> f32 {
          self.g
     }
     
     pub fn b(&self) -> f32 {
          self.b
     }

     pub fn write_color(&self) {
          let ir: i32 = (255.999 * self.r) as i32;
          let ig: i32 = (255.999 * self.g) as i32;
          let ib: i32 = (255.999 * self.b) as i32;

          println!("{} {} {} ", ir, ig, ib);
     }
}

impl Add for Color {
     type Output = Self;
     
     fn add(self, c: Color) -> Color {
          Color {
               r: self.r + c.r,
               g: self.g + c.g,
               b: self.b + c.b,
          }
     }
}

impl Sub for Color {
     type Output = Self;
     
     fn sub(self, c: Color) -> Color {
          Color {
               r: self.r - c.r,
               g: self.g - c.g,
               b: self.b - c.b,
          }
     }
}

impl Mul for Color {
     type Output = Self;
     
     fn mul(self, c: Color) -> Color {
          Color {
               r: self.r * c.r,
               g: self.g * c.g,
               b: self.b * c.b,
          }
     }
}

impl Mul<f32> for Color {
     type Output = Self;
     
     fn mul(self, t: f32) -> Color {
          Color {
               r: self.r * t,
               g: self.g * t,
               b: self.b * t,
          }
     }
}

impl Div<f32> for Color {
     type Output = Self;
     
     fn div(self, t: f32) -> Color {
          Color {
               r: self.r / t,
               g: self.g / t,
               b: self.b / t,
          }
     }
}