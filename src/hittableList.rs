use super::sphere::Sphere;
use super::hittable::Hittable;
use super::hitRecord::HitRecord;
use super::ray::Ray;

#[derive(Clone, Debug)]
pub struct HittableSpheres {
    pub spheres: Vec<Sphere>
}

impl HittableSpheres {
    pub fn hit(&self, r: &Ray, tMin: f64, tMax: f64) -> Option<HitRecord> {
        let mut closestSoFar = tMax;
        let mut optionalHitRecord: Option<HitRecord> = None;
        for sphere in &self.spheres {
            match sphere.hit(r, tMin, closestSoFar) {
                Some(hitRecord) => {
//                    println!("HitRecord is {:?}", hitRecord);
                    closestSoFar = hitRecord.t;
                    optionalHitRecord = Some(hitRecord);
                },
                None => {
//                    println!("No hitRecord for sphere {:?}", sphere);
                    ()
                }
            }
        }
        optionalHitRecord
    }
}
