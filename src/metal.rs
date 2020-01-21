use super::hitRecord::HitRecord;
use super::ray::Ray;
use super::material::Scatter;
use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(v: Vec3) -> Metal{
        Metal {
            albedo: v
        }
    }
}
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    let dot_product: f64 = 2.0 * v.dot(n);
    *v -  (*n) * dot_product
}

impl Scatter for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r.direction().unitVector(), &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}