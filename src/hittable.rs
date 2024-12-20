use crate::{interval::Interval, Ray};


trait Hittable {
    fn hit(r: &Ray, ray_t: Interval, rec: Hit_Record) -> bool;
}

struct Hit_Record {

}
impl Hittable for Hit_Record {
    fn hit(r: &Ray, ray_t: Interval, rec: Hit_Record) -> bool {
        todo!()
    }
}