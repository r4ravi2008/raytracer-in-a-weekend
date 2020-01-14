use super::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub A: Vec3,
    pub B: Vec3
}

impl Ray {
    pub fn origin(self) -> Vec3 {
        self.A
    }

    pub fn direction(self) -> Vec3 {
        self.B
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.A + t * self.B
    }
}
