
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera{
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera {
    pub fn camera() -> Camera{
        let aspect_ratio = 16.0/9.0;
        let viewport_h = 2.0;
        let viewport_w = aspect_ratio * viewport_h;
        let focal_length = 1.0;
        let horiz_vec = Vec3::new(viewport_w, 0.0, 0.0);
        let vert_vec = Vec3::new(0.0, viewport_h, 0.0);
        Camera {
            origin: Vec3::default(),
            horizontal: horiz_vec,
            vertical: vert_vec,
            lower_left_corner: Vec3::default() - horiz_vec/2.0 - vert_vec/2.0 - Vec3::new(0.0, 0.0, focal_length),
            // origin: Vec3::new(0.0, 0.0, 0.0),
            // vertical: Vec3::new(0.0, 2.0, 0.0),
            // horizontal: Vec3::new(4.0, 0.0, 0.0),
            // lower_left_corner: Vec3::new( -2.0,-1.0,-1.0)    
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
}