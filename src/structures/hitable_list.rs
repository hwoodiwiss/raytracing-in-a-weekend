use std::sync::Arc;

use crate::{
    hitable::{Hitable, RayHit},
    ray::Ray,
};

pub struct HitableList {
    list: Vec<Arc<dyn Hitable>>,
}

impl HitableList {
    pub fn new(items: &[Arc<dyn Hitable>]) -> Self {
        HitableList {
            list: Vec::from(items),
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let mut closest_so_far = t_max;
        let mut best_hit: Option<RayHit> = None;
        for hitable in &self.list {
            if let Some(hit) = hitable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.distance;
                best_hit = Some(hit);
            }
        }
        best_hit
    }
}
