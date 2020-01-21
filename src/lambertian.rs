use super::vec3::Vec3;
use super::hitRecord::HitRecord;
use super::ray::Ray;
use super::material::Scatter;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(v: Vec3) -> Lambertian {
        Lambertian{
            albedo: v
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit_record.p + hit_record.normal + super::randomInUnitSphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        Some((self.albedo, scattered))
    }
}