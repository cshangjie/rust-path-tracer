use std::f64::INFINITY;
use rand::Rng;



mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;

use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use hittable::{HitRecord, Hittable};
use hittable_list::*;
use camera::Camera;



fn color(r: &Ray, world: &HittableList, depth: i64) -> Vec3 {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Vec3::default();
    }
    if world.hit(r, 0.001, INFINITY, &mut rec){
        let target = rec.p() + rec.normal() + Vec3::random_in_unit_sphere();
        return color(&Ray::new(rec.p(), target - rec.p()), world, depth -1) * 0.5;
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = (unit_direction.y() + 1.0) * 0.5;
    return Vec3::new(1.0, 1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t;
    // if world.hit(r, 0.0, std::f64::MAX, &mut rec){
    //     return Vec3::new(
    //             rec.normal().x() + 1.0, 
    //             rec.normal().y() + 1.0, 
    //             rec.normal().z() + 1.0
    //             )* 0.5;
    // } else {
    //     let unit_direction = Vec3::unit_vector(&r.direction());
    //     let t = 0.5 * (unit_direction.y() + 1.0);

    //     Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    // }
}
 
fn main() {
    // img details
    const ASPECT_RATIO: f64 = 16.0/9.0;
    let width = 1080;
    let height = (width as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL:i64 = 100;
    const MAX_DEPTH: i64 = 50;

    // camera
    let cam = Camera::camera();
    let mut rng = rand::thread_rng();

    // world
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    let world = HittableList::new(list);
  
    // render
    println!("P3\n{} {}\n255", width, height);
    for j in (0..height).rev(){
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..width{
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0); 
            for _s in 0..SAMPLES_PER_PIXEL{
                let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / height as f64;

                let ray = &cam.get_ray(u, v);
                pixel_color = pixel_color + color(&ray, &world, MAX_DEPTH);
                //pixel_color += color(&ray, &world);
            }
            pixel_color = pixel_color/SAMPLES_PER_PIXEL as f64;
            pixel_color = Vec3::new(pixel_color.r().sqrt(), pixel_color.g().sqrt(), pixel_color.b().sqrt());
            //print_color(pixel_color, SAMPLES_PER_PIXEL);
            let ir = (256.0 * (pixel_color.r())) as i64;
            let ig = (256.0 * (pixel_color.g())) as i64;
            let ib = (256.0 * (pixel_color.b())) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.\n")
}
