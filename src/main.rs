// mod ppm;
mod vec3;
mod ray;
mod color;

use vec3::Vec3;
use ray::Ray;
use color::Color;

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin - center;
    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2.0 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, *ray);
    
    if t > 0.0 {
        let n = Vec3::unit_vector(&(ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }

    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, *ray) > 0.0 {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {

    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: i32 = 400;
    const IMG_HEIGHT: i32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as i32;

    println!("P3 {} {} 255", IMG_WIDTH, IMG_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u = i as f32 / (IMG_WIDTH - 1) as f32;
            let v = j as f32 / (IMG_HEIGHT - 1) as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);
            pixel_color.write_color();
        }
    }
}
// ppm::write_ppm();
// let u: Vec3 = Vec3::new(1f32, 3f32, 5f32);
// let v: Vec3 = Vec3::new(2f32, 5f32, 6f32);

// let sum = u + v;
// let diff = u - v;
// let prod = u * v;
// let prod2 = 2f32 * v;
// let prod3 = u * 2f32;
// let div = u / 2f32;

// println!("Sum of u and v: {:?}", sum);
// println!("Difference of u and v: {:?}", diff);
// println!("Product of u and v: {:?}", prod);
// println!("Product of 2 and v: {:?}", prod2);
// println!("Product of u and 2: {:?}", prod3);
// println!("Division of u and 2: {:?}", div);