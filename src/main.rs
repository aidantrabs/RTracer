// mod ppm;
mod vec3;
use vec3::Vec3;

fn main() {
    // ppm::write_ppm();
    let u: Vec3 = Vec3::new(1f32, 3f32, 5f32);
    let v: Vec3 = Vec3::new(2f32, 5f32, 6f32);
    
    let sum = u + v;
    let diff = u - v;
    let prod = u * v;
    let prod2 = 2f32 * v;
    let prod3 = u * 2f32;
    let div = u / 2f32;
    
    println!("Sum of u and v: {:?}", sum);
    println!("Difference of u and v: {:?}", diff);
    println!("Product of u and v: {:?}", prod);
    println!("Product of 2 and v: {:?}", prod2);
    println!("Product of u and 2: {:?}", prod3);
    println!("Division of u and 2: {:?}", div);
}