use std::sync::Arc;

use crate::{
    hitable::RayHit,
    material::{Material, MaterialHit},
    point_in_unit_sphere,
    ray::Ray,
    vec3::Vec3,
};

pub struct Diffuse {
    albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn boxed(albedo: Vec3) -> Box<Arc<dyn Material>> {
        Box::new(Arc::new(Self::new(albedo)))
    }
}

impl Material for Diffuse {
    fn scatter(&self, in_ray: &Ray, hit: &RayHit) -> Option<MaterialHit> {
        let target = hit.point + hit.normal + point_in_unit_sphere();
        let scatter_ray = Ray::new(hit.point, target);
        let attenuation = self.albedo;
        Some(MaterialHit {
            attenuation,
            scatter_ray,
        })
    }
}
