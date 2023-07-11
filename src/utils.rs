use rand::Rng;

pub static PI: f32 = 3.1415926535897932385;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
     return degrees * (PI / (180.0 as f64));
}

#[inline]
pub fn random_f32() -> f32 {
     let mut rng = rand::thread_rng();
     return rng.gen_range(0.0..1.0) as f32;
}

#[inline]
pub fn random_f32_range(min: f32, max: f32) -> f32 {
     let mut rng = rand::thread_rng();
     return rng.gen_range(min..max) as f32
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
     if x < min {
          return min;
     }
     
     if x > max {
          return max;
     }

     return x;
}