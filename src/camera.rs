use crate::vec3::Vec3;
use crate::{unit_vector, Point3};
use crate::Ray;
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
    fn new(
        aspect_ratio: f32, image_width: i32, samples_per_pixel: i32, max_depth: i32,
        v_fov : f32, look_from: Point3, look_at: Point3, v_up: Vec3, defocus_angle: f32, focus_dist: f32
    ) -> Self {
        let  image_height = image_width / aspect_ratio as i32;

        let  pixel_samples_scale = 1.0 / samples_per_pixel as f32;

        let  center = look_from;
        //viewport dimensions
        let  theta = v_fov.to_radians();
        let h = (theta/2.0).tan();
        let  viewport_height = 2.0 * h * focus_dist;
        let  viewport_width = viewport_height * (image_width/image_height) as f32;

        //v u w basis vectors for the camera coords
        let mut w = unit_vector(look_from-look_at);
        let mut u = unit_vector(todo!("cross"));
        let mut v : Vec3<>= todo!("cross ");

        let mut viewport_u = u* viewport_width;
        let mut viewport_v = (v* -1.0)* viewport_height;

        let mut pixel_delta_u = viewport_u / image_height as f32;
        let mut pixel_delta_v = viewport_v / image_height as f32;

        let mut viewport_upper_left = center - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let mut pixel100_loc = (pixel_delta_u+pixel_delta_v) *0.5 + viewport_upper_left ;

        let mut defocus_radius = (defocus_angle / 2.0).to_radians().tan() * focus_dist;
        let mut defocus_disk_u = u * defocus_radius;
        let mut defocus_disk_v = v * defocus_radius;
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            v_fov,
            look_from,
            look_at,
            v_up,
            defocus_angle,
            focus_dist,

            image_height,
            pixel_samples_scale,
            center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
            v,
            u,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
}
