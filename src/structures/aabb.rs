use std::mem::swap;

use super::{Ray, Vec3};

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for i in 0..3 {
            let inv_dir = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_dir;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_dir;
            if inv_dir < 0.0 {
                swap(&mut t0, &mut t1);
            }
            let t_min = f32::max(t0, t_min);
            let t_max = f32::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let min = Vec3::new(
            f32::min(box0.min.x(), box1.min.x()),
            f32::min(box0.min.y(), box1.min.y()),
            f32::min(box0.min.z(), box1.min.z()),
        );
        let max = Vec3::new(
            f32::max(box0.max.x(), box1.max.x()),
            f32::max(box0.max.y(), box1.max.y()),
            f32::max(box0.max.z(), box1.max.z()),
        );
        AABB { min, max }
    }
}
