use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3{
    e: [f32;3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3{
        Vec3 { e: [e0, e1, e2] }
    }
    pub fn unit_vector(v: &Vec3) -> Vec3{
        *v / v.length()
    }
    pub const fn x(&self) -> f32{
        return self.e[0];
    }
    pub const fn y(&self) -> f32{
        return self.e[1 ];
    }
    pub const fn z(&self) -> f32{
        return self.e[2];
    }
    pub fn length(self) -> f32 {
         (self.e[0]*self.e[0] + 
          self.e[1]*self.e[1] + 
          self.e[2]*self.e[2]).sqrt()
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {e: [self.e[0] + rhs.e[0],
                  self.e[1] + rhs.e[1],
                  self.e[2] + rhs.e[2]]
        }
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output{
        Vec3 {
            e: [self.e[0]*rhs, self.e[1]*rhs, self.e[2]*rhs]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output{
        let k: f32 = 1.0/rhs;
        Vec3{
            e: [self.e[0]*k, self.e[1]*k, self.e[2]*k]
        }
    }
}

// Testing
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_vec3_add(){
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) + Vec3::new(3.0, 2.0, 1.0), 
                   Vec3::new(4.0, 4.0, 4.0))
    }
    #[test]
    fn test_vec3_mul(){
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 4.0, Vec3::new(4.0, 8.0, 12.0))
    }

    #[test]
    fn test_vec3_div(){
        assert_eq!(Vec3::new(4.0, 2.0, 6.0) / 2.0, Vec3::new(2.0, 1.0, 3.0))
    }
}

