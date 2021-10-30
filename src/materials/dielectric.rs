use std::rc::Rc;

use rand::{thread_rng, Rng};

use crate::{
    hitable::RayHit,
    material::{Material, MaterialHit},
    ray::Ray,
    vec3::Vec3,
};

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    pub fn boxed(refractive_index: f32) -> Box<Rc<dyn Material>> {
        Box::new(Rc::new(Self::new(refractive_index)))
    }

    fn schlick(cosine: f32, refractive_index: f32) -> f32 {
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: &Ray, hit: &RayHit) -> Option<MaterialHit> {
        let mut rng = thread_rng();
        let reflected = in_ray.direction.relfect(&hit.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if in_ray.direction.dot(&hit.normal) > 0.0 {
            let cosine = self.refractive_index * in_ray.direction.dot(&hit.normal)
                / in_ray.direction.length();
            (-hit.normal, self.refractive_index, cosine)
        } else {
            let cosine = -in_ray.direction.dot(&hit.normal) / in_ray.direction.length();
            (hit.normal, 1.0 / self.refractive_index, cosine)
        };
        let mut refraction_dir = Vec3::new(0.0, 0.0, 0.0);
        let reflected_prob =
            if let Some(refracted) = in_ray.direction.refract(&outward_normal, ni_over_nt) {
                refraction_dir = refracted;
                Dielectric::schlick(cosine, self.refractive_index)
            } else {
                1.0
            };
        let scatter_ray = if rng.gen::<f32>() < reflected_prob {
            Ray::new(hit.point, reflected)
        } else {
            Ray::new(hit.point, refraction_dir)
        };
        Some(MaterialHit {
            attenuation,
            scatter_ray,
        })
    }
}
