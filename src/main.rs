#![allow(dead_code)]
use std::ops::Add;

#[derive(Debug)]
struct Vec3 {
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


impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{x, y, z}
    }

    pub fn x(&self) -> f32 {
        self.x
    }
}

fn main() {
    let  my_vec = Vec3::new(32.0, 32.0, 32.0);
    let  my_vec2 = Vec3::new(32.0, 32.0, 32.0);
    let  my_new_vec = my_vec.add(my_vec2);
    println!("{:?}", my_new_vec);
}
