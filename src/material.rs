use crate::{
    hitable::RayHit,
    structures::{Ray, Vec3},
};

pub struct MaterialHit {
    pub attenuation: Vec3,
    pub scatter_ray: Ray,
}
pub trait Material: Sync + Send {
    fn scatter(&self, in_ray: &Ray, hit: &RayHit) -> Option<MaterialHit>;
}
