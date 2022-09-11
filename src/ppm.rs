pub fn write_ppm() {
     const IMG_WIDTH: i32 = 256;
     const IMG_HEIGHT: i32 = 256;

     println!("P3\n{} {} \n255\n", IMG_WIDTH, IMG_HEIGHT);

     for j in (0..IMG_HEIGHT).rev() {
          for i in 0..IMG_WIDTH {
               let r: f32 = i as f32 / IMG_WIDTH as f32;
               let g: f32 = j as f32 / IMG_HEIGHT as f32;
               let b: f32 = 0.25;

               let ir: i32 = (255.999 * r) as i32;
               let ig: i32 = (255.999 * g) as i32;
               let ib: i32 = (255.999 * b) as i32;

               println!("{} {} {} ", ir, ig, ib);
          }
     }
}
