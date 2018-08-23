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
use material::{Dielectrics, Lambertian, Metal};
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
    },
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

fn random_scene() -> HitableList {
  let mut world = HitableList::with_capacity(500);
  world.push(Box::new(Sphere::new(
    Vec3::new(0.0, -1000.0, 0.0),
    1000.0,
    Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
  )));
  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = random::<f32>();
      let center = Vec3::new(
        a as f32 + 0.9 * random::<f32>(),
        0.2,
        b as f32 + 0.9 * random::<f32>(),
      );
      if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        if choose_mat < 0.8 {
          world.push(Box::new(Sphere::new(
            center,
            0.2,
            Rc::new(Lambertian::new(Vec3::new(
              random::<f32>() * random::<f32>(),
              random::<f32>() * random::<f32>(),
              random::<f32>() * random::<f32>(),
            ))),
          )));
        } else if choose_mat < 0.95 {
          world.push(Box::new(Sphere::new(
            center,
            0.2,
            Rc::new(Metal::new(
              Vec3::new(
                0.5 * (1.0 + random::<f32>()),
                0.5 * (1.0 + random::<f32>()),
                0.5 * (1.0 + random::<f32>()),
              ),
              0.5 * random::<f32>(),
            )),
          )));
        } else {
          world.push(Box::new(Sphere::new(
            center,
            0.2,
            Rc::new(Dielectrics::new(1.5)),
          )));
        }
      }
    }
  }
  world.push(Box::new(Sphere::new(
    Vec3::new(-4.0, 1.0, 0.0),
    1.0,
    Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
  )));
  world.push(Box::new(Sphere::new(
    Vec3::new(4.0, 1.0, 0.0),
    1.0,
    Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
  )));
  world.push(Box::new(Sphere::new(
    Vec3::new(0.0, 1.0, 0.0),
    1.0,
    Rc::new(Dielectrics::new(1.5)),
  )));
  world
}

fn main() {
  let nx = 600;
  let ny = 400;
  let ns = 100;
  let mut img = RgbImage::new(nx, ny);
  let world = random_scene();
  let lookfrom = Vec3::new(10.0, 2.0, 2.0);
  let lookat = Vec3::new(0.0, 0.0, -1.0);
  let cam = Camera::new(
    lookfrom,
    lookat,
    Vec3::new(0.0, 1.0, 0.0),
    25.0,
    nx as f32 / ny as f32,
    0.001,
    (lookfrom - lookat).length(),
  );
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
