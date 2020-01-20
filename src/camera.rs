use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    origin:Vec3,
    lowerLeftCorner: Vec3,
    horizontal: Vec3,
    vertical : Vec3
}

impl Camera {
    pub fn new() -> Camera{
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lowerLeftCorner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0)
        }
    }

    pub fn getRay(&self, u: f64, v:f64) -> Ray {
        Ray {
            a: self.origin,
            b: self.lowerLeftCorner + u * self.horizontal + v * self.vertical - self.origin
        }
    }
}