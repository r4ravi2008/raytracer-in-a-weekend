use super::hit_record::HitRecord;
use super::ray::Ray;
use super::vec3::Vec3;
use super::lambertian::Lambertian;
use super::metal::Metal;
use crate::dielectric::Dielectric;

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric)
}

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

impl Scatter for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        match *self {
            Material::Lambertian(ref inner) => inner.scatter(ray_in, hit_record),
            Material::Metal(ref inner) => inner.scatter(ray_in, hit_record),
            Material::Dielectric(ref inner) => inner.scatter(ray_in, hit_record),
        }
    }
}