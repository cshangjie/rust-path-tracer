use std::ops;
use rand::Rng;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3{
    e: [f64;3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3{
        Vec3 { e: [e0, e1, e2] }
    }
    pub const fn x(&self) -> f64{
        return self.e[0];
    }
    pub const fn y(&self) -> f64{
        return self.e[1 ];
    }
    pub const fn z(&self) -> f64{
        return self.e[2];
    }
    pub const fn r(&self) -> f64{
        return self.e[0];
    }
    pub const fn g(&self) -> f64{
        return self.e[1 ];
    }
    pub const fn b(&self) -> f64{
        return self.e[2];
    }
    pub fn length(self) -> f64 {
         (self.e[0]*self.e[0] + 
          self.e[1]*self.e[1] + 
          self.e[2]*self.e[2]).sqrt()
    }
    pub fn length_squared(self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        return (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s);
    }
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f64 {
        vec1.e[0] * vec2.e[0] + vec1.e[1] * vec2.e[1] + vec1.e[2] * vec2.e[2]
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::default();
        let mut rng = rand::thread_rng();
        loop{
            p = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())* 2.0 - Vec3::new(1.0, 1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
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
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self){
        *self = Self{
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2],]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {e: [self.e[0] - rhs.e[0],
                  self.e[1] - rhs.e[1],
                  self.e[2] - rhs.e[2]]
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output{
        Vec3 {
            e: [self.e[0]*rhs, self.e[1]*rhs, self.e[2]*rhs]
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output{
        let k: f64 = 1.0/rhs;
        Vec3{
            e: [self.e[0]*k, self.e[1]*k, self.e[2]*k]
        }
    }
}
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Vec3{
        Vec3{
            e: [-self.e[0], -self.e[1], -self.e[2]]
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

