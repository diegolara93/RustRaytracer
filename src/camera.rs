struct camera {
    // Values that will have a default value
    aspect_ratio : f32,
    image_width : i32,
    samples_per_pixel : i32,
    max_depth : i32,
    vFov : f32,
    lookFrom: Point3,
    lookAt: Point3, 
    vUp : Vec3,
    defocus_angle : f32,
    focus_dist : f32,

    // Private values
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

impl Default for camera {
    
}

