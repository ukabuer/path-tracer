extern crate image;
extern crate rand;

mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, HitableList, Sphere};
use image::RgbImage;
use material::{Lambertian, Metal};
use rand::random;
use ray::Ray;
use std::f32;
use std::rc::Rc;
use vec3::Vec3;

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
  let result = world.hit(r, 0.001, f32::MAX);
  match result {
    None => {
      let unit_direction = Vec3::unit_vector(r.direction());
      let t = (unit_direction.y() + 1.0) * 0.5;
      Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
    Some(rec) => {
      if depth < 50 {
        match rec.mat.scatter(r, &rec) {
          None => Vec3::new(0.0, 0.0, 0.0),
          Some((scattered, attenuation)) => attenuation * color(&scattered, world, depth + 1),
        }
      } else {
        Vec3::new(0.0, 0.0, 0.0)
      }
    }
  }
}

fn main() {
  let nx = 200;
  let ny = 100;
  let ns = 100;
  let mut img = RgbImage::new(nx, ny);
  let mut world = HitableList::new();
  world.push(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -1.0),
    0.5,
    Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
  )));
  world.push(Box::new(Sphere::new(
    Vec3::new(0.0, -100.5, -1.0),
    100.0,
    Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
  )));
  world.push(Box::new(Sphere::new(
    Vec3::new(1.0, 0.0, -1.0),
    0.5,
    Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2))),
  )));
  world.push(Box::new(Sphere::new(
    Vec3::new(-1.0, 0.0, -1.0),
    0.5,
    Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8))),
  )));
  let cam = Camera::new();
  for j in 0..ny {
    for i in 0..nx {
      let mut col = Vec3::new(0.0, 0.0, 0.0);
      for _s in 0..ns {
        let u = (i as f32 + random::<f32>()) / nx as f32;
        let v = (j as f32 + random::<f32>()) / ny as f32;
        let r = cam.get_ray(u, v);
        col += color(&r, &world, 0);
      }
      col /= ns as f32;
      col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
      let pixel = &mut img[(i, ny - j - 1)];
      pixel[0] = (255.99 * col[0]) as u8;
      pixel[1] = (255.99 * col[1]) as u8;
      pixel[2] = (255.99 * col[2]) as u8;
    }
  }
  img.save("output.png").expect("can not save image file");
}
