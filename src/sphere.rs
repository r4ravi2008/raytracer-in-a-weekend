use super::vec3::Vec3;
use super::hittable::Hittable;
use super::hitRecord::HitRecord;
use super::ray::Ray;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tMin: f64, tMax: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
//        println!("oc is {:?}", oc);
        let a = ray.direction().dot(ray.direction());
//        println!("a is {:?}", a);
        let b = oc.dot(ray.direction());
//        println!("b is {:?}", b);
        let c = oc.dot(oc) - self.radius * self.radius;
//        println!("c is {:?}", c);
        let discriminant = b * b -  a * c;
//        println!("discriminant is {:?}", discriminant);
        let mut someHitRecord: Option<HitRecord> = None;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
//            println!("temp is {}",temp);
//            println!("tmaxn and tmin is {} {}", tMax, tMin);
            if temp < tMax && temp > tMin {
                let point = ray.pointAtParameter(temp);
//                println!("point is {:?}", point);
                let hr = HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
                };
//                println!("hr is {:?}", hr);
                return Some(hr)
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < tMax && temp > tMin {
                let point = ray.pointAtParameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius,
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
