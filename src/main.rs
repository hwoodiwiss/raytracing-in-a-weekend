use std::rc::Rc;

use camera::Camera;
use hitable::Hitable;
use image::png::PngEncoder;
use rand::{thread_rng, Rng};
use ray::Ray;

#[macro_use]
extern crate impl_ops;

use crate::{hitable::HitableList, shapes::sphere::Sphere, vec3::Vec3};

mod camera;
mod hitable;
mod ray;
mod shapes;
mod vec3;

fn ray_colour(ray: &Ray, hitable: &dyn Hitable) -> Vec3 {
    if let Some(hit) = hitable.hit(ray, 0.0, f32::MAX) {
        return 0.5
            * Vec3::new(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
            );
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return &((1.0 - t) * &Vec3::new(1.0, 1.0, 1.0)) + &(t * &Vec3::new(0.5, 0.7, 1.0));
    };
}

fn main() {
    let nx = 2000;
    let ny = 1000;
    let samples = 100;

    let sphere_1: Box<Rc<dyn Hitable>> =
        Box::new(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    let sphere_2: Box<Rc<dyn Hitable>> =
        Box::new(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let world = HitableList::new(&[sphere_1, sphere_2]);
    let camera = Camera::new();
    let mut image_bytes = Vec::new();
    let mut rng = thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples {
                let u_jitter: f32 = rng.gen();
                let v_jitter: f32 = rng.gen();
                let u = (i as f32 + u_jitter) / nx as f32;
                let v = (j as f32 + v_jitter) / ny as f32;
                let ray = camera.get_ray(u, v);
                col += ray_colour(&ray, &world);
            }
            col /= samples as f32;
            let ir = (255.99 * col.r()) as u16;
            let ig = (255.99 * col.g()) as u16;
            let ib = (255.99 * col.b()) as u16;
            image_bytes.extend_from_slice(&ir.to_le_bytes());
            image_bytes.extend_from_slice(&ig.to_le_bytes());
            image_bytes.extend_from_slice(&ib.to_le_bytes());
        }
    }

    let mut file = std::fs::File::create("raytracing.png").unwrap();
    let png_encoder = PngEncoder::new(&mut file);
    png_encoder
        .encode(&image_bytes, nx, ny, image::ColorType::Rgb16)
        .unwrap();
}
