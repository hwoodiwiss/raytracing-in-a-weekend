use std::sync::Arc;

use crate::hitable::{Hitable, RayHit};

use super::{Ray, AABB};

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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.list.is_empty() {
            None
        } else {
            if let Some(mut scene_bound) = self.list[0].bounding_box(t0, t1) {
                for hitable in &self.list {
                    if let Some(item_bound) = hitable.bounding_box(t0, t1) {
                        scene_bound = AABB::surrounding_box(&item_bound, &scene_bound);
                    } else {
                        return None;
                    }
                }
                Some(scene_bound)
            } else {
                None
            }
        }
    }
}
