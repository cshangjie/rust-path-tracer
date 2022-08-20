use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable}; 

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,

}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest_so_far = t_max;

        for obj in &self.list {
            if obj.hit(r, t_min, t_max, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t();
                rec.set_p(temp_rec.p());
                rec.set_t(temp_rec.t());
                rec.set_normal(temp_rec.normal());
                // rec = &mut temp_rec;

                
            }
        }
        return hit_anything;
    }
}