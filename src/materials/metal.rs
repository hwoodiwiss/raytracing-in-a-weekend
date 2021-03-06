use std::sync::Arc;

use crate::{
    hitable::RayHit,
    material::{Material, MaterialHit},
    structures::{Ray, Vec3},
};

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }

    pub fn arc(albedo: Vec3, fuzz: f32) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo, fuzz))
    }
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit: &RayHit) -> Option<MaterialHit> {
        let reflection = in_ray.direction.unit().relfect(&hit.normal);
        let scatter_ray = Ray::new(
            hit.point,
            reflection + self.fuzz * Vec3::get_point_in_unit_sphere(),
            in_ray.time,
        );
        let attenuation = self.albedo;
        if scatter_ray.direction.dot(&hit.normal) > 0.0 {
            Some(MaterialHit {
                attenuation,
                scatter_ray,
            })
        } else {
            None
        }
    }
}
