use crate::{hitable::RayHit, ray::Ray, vec3::Vec3};

pub struct MaterialHit {
    pub attenuation: Vec3,
    pub scatter_ray: Ray,
}
pub trait Material {
    fn scatter(&self, in_ray: &Ray, hit: &RayHit) -> Option<MaterialHit>;
}
