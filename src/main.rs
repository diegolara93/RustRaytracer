#![allow(dead_code)]
use std::fmt::Display;
mod vec3;
mod camera;
mod interval;
mod hittable;
mod material;


use vec3::Vec3;


type Color = Vec3;
struct Image {
    pixels: Vec<Vec<Color>>,
    height: i32,
    width: i32,
    camera_center: Vec3,
}

impl Image {
    pub fn new(height: i32, width: i32, camera_center: Vec3) -> Image {
        let _pixels: Vec<Vec<Color>> = vec![];
            Image {
                pixels : _pixels,
                height, 
                width,
                camera_center,
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
    pub fn at(self, t: f32) -> Point3 {
        self.origin + self.direction*t
    }
}
fn main() {
    let  camera_center = Point3::new(0.0,0.0,0.0);
    let mut image = Image::new(255, 255, camera_center);
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
    if t > 0.0 {
        let N = unit_vector(r.at(t)-Vec3::new(0.0, 0.0, 0.0));
        return Color::new(N.x()+1.0, N.y() +1.0, N.z() + 1.0) * 0.5;
    }
    let unit_direction = unit_vector(r.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;
    return Color::new(1.0, 1.0, 1.0) * (1.0-a) + Color::new(0.5, 0.7, 1.0) * a;
}

fn unit_vector(v: Vec3) -> Vec3 {
    let l = v.len();
    Vec3::new(
         v.x() / l,
         v.y() / l,
         v.z() / l
    )
}

fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y() * v.z() - u.y() * v.y() , 
        u.y() * v.x() - u.x() * v.y(), 
        u.x() * v.y() - u.y() * v.x())
}
