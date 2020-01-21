use super::vec3::Vec3;
use super::hit_record::HitRecord;
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
    fn scatter(&self, _r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit_record.p + hit_record.normal + super::random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        Some((self.albedo, scattered))
    }
}