use crate::vec3::Vec3;
use crate::{unit_vector, Point3};
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
    image_height : i32,
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
    fn initialize(mut self) {
        self.image_height = self.image_width /  self.aspect_ratio as i32;
        self.pixel_samples_scale = (1 / self.samples_per_pixel) as f32;

        self.center = self.look_from;

        let theta = self.v_fov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width / self.image_height) as f32;

        self.w = unit_vector(self.look_from-self.look_at);
        self.u = todo!("setup cross product function");
        self.v = todo!("also have to do crossproduct");

        let viewport_u: Vec3 = self.u * viewport_width;
        let viewport_v: Vec3 = self.v * -1 * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left = self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel100_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
    pub fn render(self) {
        self.initialize();

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
             image_height : 0,
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

