use crate::{Vec3, Point3, Ray};
use crate::utils::{PI};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
     origin: Point3,
     lower_left_corner: Point3,
     horizontal: Vec3,
     vertical: Vec3,
     u: Vec3,
     v: Vec3,
     lens_radius: f32,
}

impl Camera {
     pub fn new(
          from: Point3,
          at: Point3,
          vup: Vec3,
          vfov: f32,
          aspect_ratio: f32,
          aperture: f32,
          focus_dist: f32,
     ) -> Camera {
          let theta = (PI as f32) / 180.0 * vfov;
          let vp_height = 2.0 * ((theta as f32) / 2.0).tan();
          let vp_width = aspect_ratio * vp_height;

          let _w = Vec3::unit_vector(from - at);
          let _u = Vec3::unit_vector(Vec3::cross(vup, _w));
          let _v = Vec3::cross(_w, _u);

          let h = focus_dist * vp_width * _u;
          let v = focus_dist * vp_height * _v;
          let llc = from - h / 2.0 - v / 2.0 - focus_dist * _w;

          return Camera {
               origin: from,
               lower_left_corner: llc,
               horizontal: h,
               vertical: v,
               u: _u,
               v: _v,
               lens_radius: aperture / 2.0,
          }
     }

     pub fn get_ray(&self, s: f32, t: f32) -> Ray {
          let rd = self.lens_radius * Vec3::random_in_unit_disk();
          let offset = rd.x() * self.u + rd.y() * self.v;

          return Ray::new(
               self.origin + offset,
               self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
          );
     }
}