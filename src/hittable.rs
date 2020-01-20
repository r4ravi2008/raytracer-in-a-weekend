use super::ray::Ray;
use super::hitRecord::HitRecord;

pub trait Hittable {
   fn hit(&self, r: &Ray, tMin: f64, tMax: f64) -> Option<HitRecord>;
}
