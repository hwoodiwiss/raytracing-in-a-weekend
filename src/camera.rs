use std::f32::consts;

use crate::{ray::Ray, vec3::Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, vert_fov: f32, aspect: f32) -> Self {
        let theta = vert_fov * consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = half_height * aspect;
        let w = (position - look_at).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);
        Self {
            lower_left_corner: position - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: position,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin,
        )
    }
}
