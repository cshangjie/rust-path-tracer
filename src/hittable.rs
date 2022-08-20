use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    p: Vec3,
    t: f64,
    normal: Vec3,
    front_face: bool
}

impl HitRecord {
    pub fn p(&self) -> Vec3{
        self.p  
    }
    pub fn t(&self) -> f64{
        self.t  
    }
    pub fn normal(&self) -> Vec3{
        self.normal  
    }
    pub fn set_p(&mut self, p_val: Vec3) {
        self.p  = p_val;
    }
    pub fn set_t(&mut self, t_val: f64) {
        self.t = t_val;
    }
    pub fn set_normal(&mut self, normal_val: Vec3) {
        self.normal = normal_val;  
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3){
        self.front_face = Vec3::dot(&ray.direction(), outward_normal) < 0.0;
        if(self.front_face){
            self.normal = *outward_normal;
        } else {
            self.normal = -(*outward_normal);
        }
    }
}
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        false
    }
}