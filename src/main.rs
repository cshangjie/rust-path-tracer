use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;

use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use hittable::{HitRecord, Hittable};
use hittable_list::*;


fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, std::f64::MAX, &mut rec){
        return Vec3::new(rec.normal().x() + 1.0, rec.normal().y() + 1.0, rec.normal().z() + 1.0)* 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
 
fn main() {

    // img details
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / ASPECT_RATIO) as i32;

    // camera


    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_w, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_h, 0.0);
    let lower_left_corner = origin - horizontal/(2 as f64) - vertical/(2 as f64) - Vec3::new(0.0, 0.0, focal_length);

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();

    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));

    let world = HittableList::new(list);
  
    // render
    println!("P3\n{} {}\n255", width, height);
    for j in (0..height).rev(){
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..width{
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;

            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r, &world);

            let ir: i32 = (255.999 * col.r()) as i32;
            let ig: i32 = (255.999 * col.g()) as i32;
            let ib: i32 = (255.999 * col.b()) as i32; 

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.\n")
}
