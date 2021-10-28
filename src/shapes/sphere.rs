use std::rc::Rc;

use crate::{
    hitable::{Hitable, RayHit},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    centre: Vec3,
    radius: f32,
    material: Box<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f32, material: Box<Rc<dyn Material>>) -> Self {
        Sphere {
            centre,
            radius,
            material,
        }
    }

    pub fn boxed(
        centre: Vec3,
        radius: f32,
        material: Box<Rc<dyn Material>>,
    ) -> Box<Rc<dyn Hitable>> {
        Box::new(Rc::new(Self::new(centre, radius, material)))
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let oc = ray.origin - self.centre;
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
                    normal: (ray.point_at(distance) - self.centre) / self.radius,
                    material: self.material.clone(),
                });
            }
            let distance = (-b + (b.powi(2) - a * c).sqrt()) / a;
            if distance < t_max && distance > t_min {
                return Some(RayHit {
                    distance,
                    point: ray.point_at(distance),
                    normal: (ray.point_at(distance) - self.centre) / self.radius,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}
