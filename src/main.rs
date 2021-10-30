use std::{rc::Rc, time::SystemTime};

use camera::Camera;
use hitable::Hitable;
use image::png::PngEncoder;
use materials::{diffuse::Diffuse, metal::Metal};
use rand::{thread_rng, Rng};
use ray::Ray;

#[macro_use]
extern crate impl_ops;

use crate::{
    hitable::HitableList, materials::dielectric::Dielectric, shapes::sphere::Sphere, vec3::Vec3,
};

mod camera;
mod hitable;
mod material;
mod materials;
mod ray;
mod shapes;
mod vec3;

fn point_in_unit_sphere() -> Vec3 {
    let mut point = Vec3::new(10.0, 10.0, 10.0);
    let mut rng = thread_rng();
    while point.length_squared() >= 1.0 {
        point = 2.0 * Vec3::new(rng.gen(), rng.gen(), rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    }
    point
}

fn ray_colour(ray: &Ray, hitable: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(hit) = hitable.hit(ray, 0.0001, f32::MAX) {
        if depth < 50 {
            if let Some(mat_hit) = hit.material.scatter(&ray, &hit) {
                return mat_hit.attenuation * ray_colour(&mat_hit.scatter_ray, hitable, depth + 1);
            }
        }
        return Vec3::new(0.0, 0.0, 0.0);
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    };
}

fn main() {
    let nx = 1920;
    let ny = 1080;
    let samples = 100;

    let sphere_1: Box<Rc<dyn Hitable>> = Sphere::boxed(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Diffuse::boxed(Vec3::new(0.8, 0.3, 0.3)),
    );
    let sphere_2: Box<Rc<dyn Hitable>> = Sphere::boxed(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Metal::boxed(Vec3::new(0.8, 0.8, 0.8), 0.0),
    );
    let sphere_3: Box<Rc<dyn Hitable>> =
        Sphere::boxed(Vec3::new(1.0, 0.0, -1.0), 0.45, Dielectric::boxed(1.5));
    let sphere_4: Box<Rc<dyn Hitable>> = Sphere::boxed(
        Vec3::new(1.2, 0.0, -2.1),
        0.45,
        Diffuse::boxed(Vec3::new(0.6, 0.8, 0.6)),
    );
    let sphere_5: Box<Rc<dyn Hitable>> = Sphere::boxed(
        Vec3::new(0.0, 0.0, 0.500000001),
        0.45,
        Metal::boxed(Vec3::new(1.0, 1.0, 1.0), 0.01),
    );
    let ground_sphere: Box<Rc<dyn Hitable>> = Sphere::boxed(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Diffuse::boxed(Vec3::new(0.8, 0.8, 0.0)),
    );

    let world = HitableList::new(&[
        sphere_1,
        sphere_2,
        sphere_3,
        sphere_4,
        sphere_5,
        ground_sphere,
    ]);
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        nx as f32 / ny as f32,
    );
    let mut image_bytes = Vec::with_capacity(nx as usize * ny as usize);
    let mut rng = thread_rng();

    let now = SystemTime::now();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..samples {
                let u_jitter: f32 = rng.gen();
                let v_jitter: f32 = rng.gen();
                let u = (i as f32 + u_jitter) / nx as f32;
                let v = (j as f32 + v_jitter) / ny as f32;
                let ray = camera.get_ray(u, v);
                col += ray_colour(&ray, &world, 0);
            }
            col /= samples as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (65534.99 * col.r()) as u16;
            let ig = (65534.99 * col.g()) as u16;
            let ib = (65534.99 * col.b()) as u16;
            image_bytes.extend_from_slice(&ir.to_be_bytes());
            image_bytes.extend_from_slice(&ig.to_be_bytes());
            image_bytes.extend_from_slice(&ib.to_be_bytes());
        }
    }
    println!(
        "Image rendered in {} seconds",
        now.elapsed().unwrap().as_secs()
    );
    let mut file = std::fs::File::create("raytracing.png").unwrap();
    let png_encoder = PngEncoder::new(&mut file);
    png_encoder
        .encode(&image_bytes, nx, ny, image::ColorType::Rgb16)
        .unwrap();
}
