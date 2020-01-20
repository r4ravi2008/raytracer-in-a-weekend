use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray{a:a, b:b}
    }
    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn pointAtParameter(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
}

impl PartialEq for Ray {
    fn eq(&self, other: &Ray) -> bool {
        self.origin() == other.origin() && self.direction() == other.direction()
    }
}

#[test]
fn test_origin() {
    let origin: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let direction: Vec3 = Vec3::new(3.0, 4.0, 1.0);
    let ray: Ray = Ray::new(origin, direction);
    assert_eq!(ray.origin(), origin);
}

#[test]
fn test_direction() {
    let origin: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let direction: Vec3 = Vec3::new(3.0, 4.0, 1.0);
    let ray: Ray = Ray::new(origin, direction);
    assert_eq!(ray.direction(), direction);
}

#[test]
fn test_param() {
    let origin: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let direction: Vec3 = Vec3::new(3.0, 4.0, 1.0);
    let ray: Ray = Ray::new(origin, direction);
    let point: Vec3 = Vec3::new(7.0, 10.0, 5.0);
    assert_eq!(ray.pointAtParameter(2.0), point);
}

