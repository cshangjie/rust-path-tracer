use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray{
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {a, b}
    }
    pub fn origin(self) -> Vec3 {
        self.a
    }
    pub fn direction(self) -> Vec3 {
        self.b
    }
    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}
 
// Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray_point_at_paramter(){
        assert_eq!(Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 2.0, 3.0)).point_at_parameter(3.0), Vec3::new(4.0, 7.0, 10.0))
    }
}