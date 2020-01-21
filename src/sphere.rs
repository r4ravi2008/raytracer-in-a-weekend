use super::vec3::Vec3;
use super::hittable::Hittable;
use super::hitRecord::HitRecord;
use super::ray::Ray;
use super::material::Material;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tMin: f64, tMax: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b -  a * c;
        let mut someHitRecord: Option<HitRecord> = None;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < tMax && temp > tMin {
                let point = ray.pointAtParameter(temp);
                let hr = HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.clone()
                };
                return Some(hr)
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < tMax && temp > tMin {
                let point = ray.pointAtParameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.clone()
                })
            }
        }
        someHitRecord
    }
}

#[test]
fn test_hit() {
    let sphereCenter = Vec3::new(0.0, 0.0, -1.0);
    let sphere = Sphere::new(sphereCenter, 0.5);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let ray = Ray::new(origin,sphereCenter);
    let optionalHitRecord = sphere.hit(&ray, 0.0, std::f64::MAX);
    println!("Optional HitRecords is {:?}", optionalHitRecord);
    assert!(optionalHitRecord.is_some())
}
