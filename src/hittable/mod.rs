use crate::common::ray::Ray;

use self::hit_record::HitRecord;

pub mod hit_record;
pub mod hittable_list;
pub mod sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
