use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: f32, 
    y: f32,
    z: f32
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: i32) -> Vec3 {
        Vec3{
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    
    fn div(self, rhs: f32) -> Vec3 {
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3: x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}


impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{x, y, z}
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn dot(&self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn len(&self) -> f32 {
        self.x.sqrt() + self.y.sqrt() + self.z.sqrt().sqrt()  // sqrt(x^2 + y^2 + z^2) is return value
    }
}