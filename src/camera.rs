use std::f32::consts;

use rand::{thread_rng, Rng};

use crate::structures::{Ray, Vec3};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
        vert_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let theta = vert_fov * consts::PI / 180.0;
        let half_height = f32::tan(theta / 2.0);
        let half_width = half_height * aspect;
        let w = (position - look_at).unit();
        let u = up.cross(&w).unit();
        let v = w.cross(&u);
        Self {
            lower_left_corner: position
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: position,
            lens_radius: aperture / 2.0,
            u,
            v,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        let mut rng = thread_rng();
        let rd = self.lens_radius * Vec3::get_point_in_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = self.time0 + rng.gen::<f32>() * (self.time1 - self.time0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (x * self.horizontal) + (y * self.vertical)
                - self.origin
                - offset,
            time,
        )
    }
}
