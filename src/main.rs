use std::{
    mem::size_of,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use camera::Camera;
use hitable::Hitable;
use image::png::PngEncoder;
use materials::{Diffuse, Metal};
use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use structures::{HitableList, Ray, Vec3};

#[macro_use]
extern crate impl_ops;

use crate::{materials::Dielectric, shapes::Sphere};

mod camera;
mod hitable;
mod material;
mod materials;
mod shapes;
mod structures;

fn random_scene() -> HitableList {
    let n = 500;
    let mut rng = thread_rng();
    let mut list = Vec::<Arc<dyn Hitable>>::with_capacity(n + 1);
    list.push(Sphere::arc(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Diffuse::arc(Vec3::new(0.5, 0.5, 0.5)),
    ));
    let headliners_plane = Vec3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let mat_choice = rng.gen::<f32>();
            let centre = Vec3::new(
                a as f32 + 0.9 + rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 + rng.gen::<f32>(),
            );
            if (centre - headliners_plane).length() > 0.9 {
                let mat = if mat_choice < 0.8 {
                    Diffuse::arc(Vec3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    ))
                } else if mat_choice < 0.95 {
                    Metal::arc(
                        Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ),
                        0.5 * rng.gen::<f32>(),
                    )
                } else {
                    Dielectric::arc(1.5)
                };
                list.push(Sphere::arc(centre, 0.2, mat));
            }
        }
    }

    list.push(Sphere::arc(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::arc(1.5),
    ));

    list.push(Sphere::arc(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Diffuse::arc(Vec3::new(0.8, 0.8, 0.8)),
    ));

    list.push(Sphere::arc(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::arc(Vec3::new(0.7, 0.6, 0.5), 0.0),
    ));

    HitableList::new(&list[..])
}

fn ray_colour(ray: &Ray, hitable: &dyn Hitable, depth: i32) -> Vec3 {
    if let Some(hit) = hitable.hit(ray, 0.0001, f32::MAX) {
        if depth < 50 {
            if let Some(mat_hit) = hit.material.scatter(ray, &hit) {
                return mat_hit.attenuation * ray_colour(&mat_hit.scatter_ray, hitable, depth + 1);
            }
        }
        Vec3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 1920;
    let ny = 1080;
    let num_pixels = nx * ny;
    let pixel_size = size_of::<u16>() * 3;
    let samples = 100;

    let world = random_scene();
    let cam_pos = Vec3::new(7.0, 1.75, 1.75);
    let cam_target = Vec3::new(4.0, 1.0, 0.0);
    let cam_focus_dist = (cam_pos - cam_target).length();
    let camera = Camera::new(
        cam_pos,
        cam_target,
        Vec3::new(0.0, 1.0, 0.0),
        70.0,
        nx as f32 / ny as f32,
        0.06,
        cam_focus_dist,
        1.0,
        2.0,
    );
    let image_bytes = Arc::new(Mutex::new(vec![0; num_pixels as usize * pixel_size]));

    let now = SystemTime::now();
    (0..num_pixels).into_par_iter().for_each(|idx| {
        let mut rng = thread_rng();
        let j = ny - idx / nx;
        let i = idx % nx;
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..samples {
            let u_jitter: f32 = rng.gen();
            let v_jitter: f32 = rng.gen();
            let u = (i as f32 + u_jitter) / nx as f32;
            let v = (j as f32 + v_jitter) / ny as f32;
            let ray = camera.get_ray(u, v);
            col += ray_colour(&ray, &world, 0);
        }
        col /= samples as f32;
        col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
        let hdr_rgb = [
            (65534.99 * col.r()) as u16,
            (65534.99 * col.g()) as u16,
            (65534.99 * col.b()) as u16,
        ];
        let mut ctr = 0;
        hdr_rgb
            .iter()
            .flat_map(|channel| channel.to_be_bytes())
            .for_each(|byte| {
                image_bytes.lock().unwrap()[idx * pixel_size + ctr] = byte;
                ctr += 1
            });
    });
    let elapsed_millis = now.elapsed().unwrap().as_millis();
    if elapsed_millis > 1000 {
        println!(
            "Image rendered in {} seconds",
            elapsed_millis as f32 / 1000.0
        );
    } else {
        println!("Image rendered in {} milliseconds", elapsed_millis);
    }
    let mut file = std::fs::File::create("raytracing.png").unwrap();
    let png_encoder = PngEncoder::new(&mut file);
    png_encoder
        .encode(
            &image_bytes.lock().unwrap(),
            nx as u32,
            ny as u32,
            image::ColorType::Rgb16,
        )
        .unwrap();
}
