use std::ops::{Add, AddAssign, Sub, Mul, Div};

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

     pub fn write_color(&self, samples_per_pixel: i32) {
          let scale = 1.0 / samples_per_pixel as f32;
          let r = (self.r * scale).sqrt();
          let g = (self.g * scale).sqrt();
          let b = (self.b * scale).sqrt();

          let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
          let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
          let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

          println!("{} {} {}", ir, ig, ib);
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

impl AddAssign for Color {
     fn add_assign(&mut self, c: Color) {
          *self = Color {
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

impl Mul<Color> for f32 {
     type Output = Color;
     
     fn mul(self, c: Color) -> Color {
          Color {
               r: self * c.r,
               g: self * c.g,
               b: self * c.b,
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
