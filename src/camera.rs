use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
     pub fn camera(
          lookfrom: Vec3,
          lookat: Vec3,
          vup: Vec3,
          vfov: f32, 
          aspect: f32,
          aperture: f32,
          focus_dist: f32,
     ) -> Camera {
          let theta = vfov * std::f32::consts::PI / 180.0;
          let half_height = (theta / 2.0).tan();
          let half_width = aspect * half_height;

          let w = (lookfrom - lookat).unit_vector();
          let u = vup.cross(w).unit_vector();
          let v = w.cross(u);

          let origin = lookfrom;
          let horizontal = focus_dist * 2.0 * half_width * u;
          let vertical = focus_dist * 2.0 * half_height * v;
          let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
          let lens_radius = aperture / 2.0;

          Camera {
               origin,
               lower_left_corner,
               horizontal,
               vertical,
               u,
               v,
               w,
               lens_radius,
          } 
     }

     pub fn get_ray(&self, s: f32, t: f32) -> Ray {
          let rd = self.lens_radius * random_in_unit_disk();
          let offset = self.u * rd.x() + self.v * rd.y();
        
          Ray {
               origin: self.origin + offset,
               direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
          }
     }
}

fn random_in_unit_disk() -> Vec3 {
     let mut p = Vec3::new(1.0, 1.0, 0.0);
     
     while p.squared_length() >= 1.0 {
          p = 2.0 * Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
     }

     return p;
}
