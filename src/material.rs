use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}


pub fn scatter (material: &Material, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
     match material {
          &Material::Lambertian { albedo } => {
               let target = rec.p + rec.normal + random_in_unit_sphere();
               *scattered = Ray::new(rec.p, target - rec.p);
               *attenuation = albedo;
               true
          }

          &Material::Metal { albedo, fuzz } => {
               let reflected = reflect(r_in.direction.unit_vector(), rec.normal);
               *scattered = Ray::new(rec.p, reflected + fuzz * random_in_unit_sphere());
               *attenuation = albedo;
               scattered.direction.dot(rec.normal) > 0.0
          }

          &Material::Dielectric { ref_idx } => {
               let mut outward_normal = Vec3::new(0.0, 0.0, 0.0);
               let reflected = reflect(r_in.direction, rec.normal);
               let mut ni_over_nt = 0.0;
               *attenuation = Vec3::new(1.0, 1.0, 1.0);
               let mut refracted = Vec3::new(0.0, 0.0, 0.0);
               let mut reflect_prob = 0.0;
               let mut cosine = 0.0;

               if Vec3::dot(&r_in.direction, &rec.normal) > 0.0 {
                    outward_normal = -rec.normal;
                    ni_over_nt = ref_idx;
                    cosine = ref_idx * Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
               } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0 / ref_idx;
                    cosine = -Vec3::dot(&r_in.direction, &rec.normal) / r_in.direction.length();
               }

               if refract(&r_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
                    reflect_prob = schlick(cosine, ref_idx);
               } else {
                    *scattered = Ray::new(rec.p, reflected);
                    reflect_prob = 1.0;
               }

               if random::<f32>() < reflect_prob {
                    *scattered = Ray::new(rec.p, reflected);
               } else {
                    *scattered = Ray::new(rec.p, refracted);
               }

               true
          }
     }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
     v - 2.0 * Vec3::dot(&v, &n) * n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
     let uv = v.unit_vector();
     let dt = Vec3::dot(&uv, n);
     let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
     if discriminant > 0.0 {
          *refracted = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();
          true
     } else {
          false
     }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
     let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
     r0 = r0 * r0;
     r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn random_in_unit_sphere() -> Vec3 {
     let mut p = Vec3::new(0.0, 0.0, 0.0);
     loop {
          p = 2.0 * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()) - Vec3::new(1.0, 1.0, 1.0);
          if p.squared_length() < 1.0 {
               break;
          }
     }
     p
}