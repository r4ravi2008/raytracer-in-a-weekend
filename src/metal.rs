use super::hit_record::HitRecord;
use super::ray::Ray;
use super::material::Scatter;
use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(v: Vec3, f: f64) -> Metal{
        Metal {
            albedo: v,
            fuzz: if f < 1.0 {f} else {1.0}
        }
    }
}
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    let dot_product: f64 = 2.0 * v.dot(n);
    *v -  (*n) * dot_product
}

impl Scatter for Metal {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + self.fuzz * super::random_in_unit_sphere());
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}