use super::sphere::Sphere;
use super::hittable::Hittable;
use super::hit_record::HitRecord;
use super::ray::Ray;

#[derive(Clone, Debug)]
pub struct HittableSpheres {
    pub spheres: Vec<Sphere>
}

impl Hittable for HittableSpheres {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut optional_hit_record: Option<HitRecord> = None;
        for sphere in &self.spheres {
            match sphere.hit(r, t_min, closest_so_far) {
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    optional_hit_record = Some(hit_record);
                }
                None => {
                    ()
                }
            }
        }
        optional_hit_record
    }
}
