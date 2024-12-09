#![allow(dead_code)]
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::fmt::Display;


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
type Color = Vec3;
struct Image {
    pixels: Vec<Vec<Color>>,
    height: i32,
    width: i32,
}

impl Image {
    pub fn new(height: i32, width: i32) -> Image {
        let _pixels: Vec<Vec<Color>> = vec![];
            Image {
                pixels : _pixels,
                height, 
                width
            }
    }
    pub fn create_image(&mut self) {
        println!("P3");
        println!("{} {}", self.height, self.width);
        println!("255");
        for i in 0..self.height {
            for j in 0..self.width {
                let mut temp_vec: Vec<Color> = vec![];
                temp_vec.push(Color::new(i as f32, j as f32, 84.0));
                self.pixels.push(temp_vec);
                println!("{} {} {}", i, j, 84);
            }
        }
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image has height {} and width {}", self.height, self.width)
    }
}

type Point3 = Vec3;
#[derive(Debug, PartialEq, Clone, Copy)]
struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn origin(self) -> Point3 {
        self.origin
    }
    pub fn direction(self) -> Vec3 {
        self.direction
    }
}
fn main() {
    let mut camera_center = Point3::new(0.0,0.0,0.0);
    let mut image = Image::new(255, 255);
    image.create_image();
}

fn hit_sphere(center: Point3, radius: f32, r: Ray) -> f32 {
        let oc = center - r.origin;
        let a = r.direction().dot(r.direction());
        let b = -2.0 * r.direction().dot(oc);
        let c = oc.dot(oc) - radius*radius;

        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            return -1.0;
        }
        else {
            return -b - discriminant.sqrt() / 2.0*a;
        }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    todo!();
}

fn unit_vector(v: &Vec3) -> Vec3 {
    let l = v.len();
    Vec3 {
        x : v.x / l,
        y : v.y / l,
        z : v.z / l
    }
}
