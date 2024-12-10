use crate::vec3::Vec3;
use crate::Point3;
struct Camera {
    // Values that will have a set default value
    aspect_ratio : f32,
    image_width : i32,
    samples_per_pixel : i32,
    max_depth : i32,
    v_fov : f32,
    look_from: Point3,
    look_at: Point3, 
    v_up : Vec3,
    defocus_angle : f32,
    focus_dist : f32,

    // Calculated values
    image_height : f32,
    pixel_samples_scale : f32, 
    center : Point3,
    pixel100_loc : Point3,
    pixel_delta_u : Vec3,
    pixel_delta_v : Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u : Vec3,
    defocus_disk_v : Vec3
}

impl Camera {
    fn initialize() {

    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
             aspect_ratio: 1.0, 
             image_width: 100, 
             samples_per_pixel: 10, 
             max_depth: 10, 
             v_fov: 90.0, 
             look_from: Point3::new(0.0, 0.0, 0.0), 
             look_at: Point3::new(0.0, 0.0, -1.0), 
             v_up: Vec3::new(0.0, 1.0, 0.0), 
             defocus_angle: 0.0, 
             focus_dist: 10.0, 
             image_height : 0.0,
             pixel_samples_scale : 0.0, 
             center : Point3::new(0.0,0.0,0.0),
             pixel100_loc : Point3::new(0.0,0.0,0.0),
             pixel_delta_u : Vec3::new(0.0,0.0,0.0),
             pixel_delta_v : Vec3::new(0.0,0.0,0.0),
             u: Vec3::new(0.0,0.0,0.0),
             v: Vec3::new(0.0,0.0,0.0),
             w: Vec3::new(0.0,0.0,0.0),
             defocus_disk_u : Vec3::new(0.0,0.0,0.0),
             defocus_disk_v : Vec3::new(0.0,0.0,0.0)
            }
    }
}

