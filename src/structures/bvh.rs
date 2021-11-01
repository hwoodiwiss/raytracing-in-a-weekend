use std::{cmp::Ordering, sync::Arc};

use rand::{thread_rng, Rng};

use crate::hitable::Hitable;

use super::AABB;

pub struct BvhNode {
    left: Arc<dyn Hitable>,
    right: Arc<dyn Hitable>,
    bounding_box: AABB,
}

impl BvhNode {
    pub fn new(items: &[Arc<dyn Hitable>], time0: f32, time1: f32) -> Self {
        let mut rng = thread_rng();
        let axis: i32 = (3.0 * rng.gen::<f32>()) as i32;
        let mut items = Vec::from(items);
        if axis == 0 {
            items.sort_by(|a, b| BvhNode::box_x_compare(a, b))
        } else if axis == 1 {
            items.sort_by(|a, b| BvhNode::box_y_compare(a, b))
        } else {
            items.sort_by(|a, b| BvhNode::box_z_compare(a, b))
        };
        let (left, right) = if items.len() == 1 {
            (items[0].clone(), items[0].clone())
        } else if items.len() == 2 {
            (items[0].clone(), items[1].clone())
        } else {
            (
                Arc::new(BvhNode::new(&items[0..items.len() / 2], time0, time1))
                    as Arc<dyn Hitable>,
                Arc::new(BvhNode::new(
                    &items[(items.len() / 2)..items.len()],
                    time0,
                    time1,
                )) as Arc<dyn Hitable>,
            )
        };
        let bounding_box = if let (Some(left_box), Some(right_box)) = (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            AABB::surrounding_box(&left_box, &right_box)
        } else {
            panic!("All Hitables in a BVH must be able to produce a bounding box")
        };
        Self {
            left,
            right,
            bounding_box,
        }
    }

    fn box_x_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        if let (Some(a_box), Some(b_box)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
            if a_box.min.x() - b_box.min.x() < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            panic!("");
        }
    }

    fn box_y_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        if let (Some(a_box), Some(b_box)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
            if a_box.min.y() - b_box.min.y() < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            panic!("");
        }
    }

    fn box_z_compare(a: &Arc<dyn Hitable>, b: &Arc<dyn Hitable>) -> Ordering {
        if let (Some(a_box), Some(b_box)) = (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
            if a_box.min.z() - b_box.min.z() < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            panic!("");
        }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, ray: &super::Ray, t_min: f32, t_max: f32) -> Option<crate::hitable::RayHit> {
        if self.bounding_box.hit(ray, t_min, t_max) {
            let left_hit = self.left.hit(ray, t_min, t_max);
            let right_hit = self.right.hit(ray, t_min, t_max);
            match (left_hit, right_hit) {
                (Some(left), Some(right)) => {
                    if left.distance < right.distance {
                        Some(left)
                    } else {
                        Some(right)
                    }
                }
                (None, Some(right)) => Some(right),
                (Some(left), None) => Some(left),
                (None, None) => None,
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.bounding_box)
    }
}
