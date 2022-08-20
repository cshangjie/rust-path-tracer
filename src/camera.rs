use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera{
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(){
        let aspect_ratio: f64 = 16.0 / 9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.0;
        
    }

    pub fn get_ray(u: f64, v: f64){
        
    }
}