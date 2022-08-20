use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::*;

pub struct Sphere {
    center: Vec3,
    radius: f64
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut  HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discrim = half_b * half_b - a*c;

        if discrim < 0.0{
            return false;
        }
        let sqrtd = discrim.sqrt();

        // closet root in range
        let mut root = (-half_b -sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.set_t(root);
        rec.set_p(r.point_at_parameter(rec.t()));
        rec.set_normal((rec.p() - self.center) / self.radius);
        
        return true;
    }
}