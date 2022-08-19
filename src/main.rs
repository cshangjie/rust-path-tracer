use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod ray;

use ray::Ray;
use vec3::Vec3;

fn create_ppm(width: i32, height: i32){

    println!("P3\n{} {}\n255", width, height);
    for j in (0..height).rev(){
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..width{
            let r: f32 = i as f32 / width as f32;
            let g: f32 = j as f32 / height as f32;
            let b: f32 = 0.2;

            let ir: i64 = (255.999 * r) as i64;
            let ig: i64 = (255.999 * g) as i64;
            let ib: i64 = (255.999 * b) as i64; 

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.\n")
}
fn main() {

    let width = 256;
    let height = 256;
    create_ppm(width, height);

    // let v: () = Vec3::new();

}
