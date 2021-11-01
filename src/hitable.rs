use std::sync::Arc;

use crate::{
    material::Material,
    structures::{Ray, Vec3, AABB},
};

pub struct RayHit {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

pub trait Hitable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
