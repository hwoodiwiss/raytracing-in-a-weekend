use crate::{
    hitable::{Hitable, RayHit},
    vec3::Vec3,
};

pub struct Cube {
    origin: Vec3,
    size: f32,
}

impl Cube {
    pub fn new(origin: Vec3, size: f32) -> Self {
        Self { origin, size }
    }
}

impl Hitable for Cube {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        todo!()
    }
}
