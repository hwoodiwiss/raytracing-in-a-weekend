use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn point_at(&self, distance: f32) -> Vec3 {
        self.origin + (self.direction * distance)
    }
}
