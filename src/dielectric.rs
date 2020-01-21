use super::material::Material;
use super::material::Scatter;
use super::ray::Ray;
use super::hit_record::HitRecord;
use super::vec3::Vec3;
use crate::metal::reflect;
use std::panic::resume_unwind;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    refract_index: f64
}

impl Dielectric {
    pub fn new(index: f64) -> Dielectric {
        Dielectric {
            refract_index: index
        }
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * *n) - *n * discriminant.sqrt())
    } else {
        None
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&ray_in.direction(), &hit_record.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt) = if ray_in.direction().dot(&hit_record.normal) > 0.0 {
            (-1.0 * hit_record.normal, self.refract_index)
        } else {
            (hit_record.normal, 1.0/ self.refract_index)
        };

        let scattered : Option<(Vec3, Ray)>= refract(&ray_in.direction(), &outward_normal, ni_over_nt)
            .map(|x| (attenuation, Ray::new(hit_record.p, x)));
        scattered
    }
}



