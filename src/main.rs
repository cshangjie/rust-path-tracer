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



fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, std::f64::MAX, &mut rec){
        return Vec3::new(
                rec.normal().x() + 1.0, 
                rec.normal().y() + 1.0, 
                rec.normal().z() + 1.0
                )* 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
// fn print_color(color: Vec3, samples_per_pixel: i64){
//     let mut r = color.r();
//     let mut g = color.g();
//     let mut b = color.b();

//         // Divide the color by the number of samples.
//         let scale = 1.0 / samples_per_pixel as f64;
//         r *= scale;
//         g *= scale;
//         b *= scale;

//         // Print out the translated [0,255] value of each color component to stdio.
//         //println!("{} {} {}", r*256.0, g*256.0, b*256.0);
//         println!("{} {} {}", (256.0 * r.clamp(0.0, 0.999)) as i64, (256.0 * g.clamp(0.0, 0.999)) as i64, (256.0 * b.clamp(0.0, 0.999)) as i64);
// }
 
fn main() {
    // img details
    const ASPECT_RATIO: f64 = 16.0/9.0;
    let width = 400;
    let height = (width as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL:i64 = 100;

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
                pixel_color = pixel_color + color(&ray, &world);
                //pixel_color += color(&ray, &world);
            }
            pixel_color = pixel_color/SAMPLES_PER_PIXEL as f64;

            //print_color(pixel_color, SAMPLES_PER_PIXEL);
            let ir = (255.999 * pixel_color.r()) as i64;
            let ig = (255.999 * pixel_color.g()) as i64;
            let ib = (255.999 * pixel_color.b()) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.\n")
}
