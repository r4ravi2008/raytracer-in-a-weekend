use super::sphere::Sphere;
use super::hittable::Hittable;
use super::hitRecord::HitRecord;
use super::ray::Ray;

#[derive(Clone, Debug)]
pub struct HittableSpheres {
    pub spheres: Vec<Sphere>
}

impl Hittable for HittableSpheres {
    fn hit(&self, r: &Ray, tMin: f64, tMax: f64) -> Option<HitRecord> {
        let mut closestSoFar = tMax;
        let mut optionalHitRecord: Option<HitRecord> = None;
        for sphere in &self.spheres {
            match sphere.hit(r, tMin, closestSoFar) {
                Some(hitRecord) => {
                    closestSoFar = hitRecord.t;
                    optionalHitRecord = Some(hitRecord);
                },
                None => {
                    ()
                }
            }
        }
        optionalHitRecord
    }
}
