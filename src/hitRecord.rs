use super::vec3::Vec3;
use super::material::Material;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material
}
