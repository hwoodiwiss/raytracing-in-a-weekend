use std::sync::Arc;

use crate::{
    hitable::{Hitable, RayHit},
    material::Material,
    structures::{Ray, Vec3},
};

pub struct MovingSphere {
    centre0: Vec3,
    centre1: Vec3,
    radius: f32,
    time0: f32,
    time1: f32,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        centre0: Vec3,
        centre1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            centre0,
            centre1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn arc(
        centre0: Vec3,
        centre1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Arc<dyn Hitable> {
        Arc::new(Self::new(centre0, centre1, time0, time1, radius, material))
    }

    pub fn centre(&self, time: f32) -> Vec3 {
        self.centre0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.centre1 - self.centre0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let oc = ray.origin - self.centre(ray.time);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let descriminant = b.powi(2) - a * c;
        if descriminant > 0.0 {
            let distance = (-b - (b.powi(2) - a * c).sqrt()) / a;
            if distance < t_max && distance > t_min {
                return Some(RayHit {
                    distance,
                    point: ray.point_at(distance),
                    normal: (ray.point_at(distance) - self.centre(ray.time)) / self.radius,
                    material: self.material.clone(),
                });
            }
            let distance = (-b + (b.powi(2) - a * c).sqrt()) / a;
            if distance < t_max && distance > t_min {
                return Some(RayHit {
                    distance,
                    point: ray.point_at(distance),
                    normal: (ray.point_at(distance) - self.centre(ray.time)) / self.radius,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}
