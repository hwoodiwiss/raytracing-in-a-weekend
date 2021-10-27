use std::rc::Rc;

use hitable::Hitable;
use ray::Ray;

#[macro_use]
extern crate impl_ops;

use crate::{hitable::HitableList, shapes::sphere::Sphere, vec3::Vec3};

mod hitable;
mod ray;
mod shapes;
mod vec3;

fn ray_colour(ray: &Ray, hitable: &dyn Hitable) -> Vec3 {
    if let Some(hit) = hitable.hit(ray, 0.0, f32::MAX) {
        return 0.5
            * Vec3::from_values(
                hit.normal.x() + 1.0,
                hit.normal.y() + 1.0,
                hit.normal.z() + 1.0,
            );
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return &((1.0 - t) * &Vec3::from_values(1.0, 1.0, 1.0))
            + &(t * &Vec3::from_values(0.5, 0.7, 1.0));
    };
}

fn main() {
    let nx = 2000;
    let ny = 1000;
    let lower_left_corner = Vec3::from_values(-2.0, -1.0, -1.0);
    let horizontal = Vec3::from_values(4.0, 0.0, 0.0);
    let vertical = Vec3::from_values(0.0, 2.0, 0.0);
    let origin = Vec3::from_values(0.0, 0.0, 0.0);
    let sphere_1: Box<Rc<dyn Hitable>> =
        Box::new(Rc::new(Sphere::new(Vec3::from_values(0.0, 0.0, -1.0), 0.5)));
    let sphere_2: Box<Rc<dyn Hitable>> = Box::new(Rc::new(Sphere::new(
        Vec3::from_values(0.0, -100.5, -1.0),
        100.0,
    )));

    let world = HitableList::new(&[sphere_1, sphere_2]);
    print!("P3\n{} {} \n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let direction = lower_left_corner + (u * horizontal) + (v * vertical);
            let ray = Ray::from_values(origin, direction);
            let col = ray_colour(&ray, &world);
            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
