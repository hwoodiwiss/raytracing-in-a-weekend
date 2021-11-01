use std::sync::Arc;

use crate::{
    hitable::RayHit,
    material::{Material, MaterialHit},
    structures::{Ray, Vec3},
};

pub struct Diffuse {
    albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn arc(albedo: Vec3) -> Arc<dyn Material> {
        Arc::new(Self::new(albedo))
    }
}

impl Material for Diffuse {
    fn scatter(&self, in_ray: &Ray, hit: &RayHit) -> Option<MaterialHit> {
        let target = hit.normal + Vec3::get_point_in_unit_sphere();
        let scatter_ray = Ray::new(hit.point, target, in_ray.time);
        let attenuation = self.albedo;
        Some(MaterialHit {
            attenuation,
            scatter_ray,
        })
    }
}
