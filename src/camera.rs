extern crate rand;

use super::{ray::Ray, vec3::Vec3};
use rand::random;
use std::f32;

fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = Vec3::new(random::<f32>(), random::<f32>(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
    if p.dot(&p) < 1.0 {
      break p;
    }
  }
}

pub struct Camera {
  pub origin: Vec3,
  pub lower_left_corner: Vec3,
  pub horizontal: Vec3,
  pub vertical: Vec3,
  pub lens_radius: f32,
  pub u: Vec3,
  pub v: Vec3,
  pub w: Vec3,
}

impl Camera {
  pub fn new(
    lookfrom: Vec3,
    lookat: Vec3,
    vup: Vec3,
    vfov: f32,
    aspect: f32,
    aperture: f32,
    focus_dist: f32,
  ) -> Camera {
    let theta = vfov * f32::consts::PI / 180.0;
    let half_height = (theta / 2.0).tan();
    let half_width = aspect * half_height;
    let w = Vec3::unit_vector(&(lookfrom - lookat));
    let u = Vec3::unit_vector(&vup.cross(w));
    let v = w.cross(u);
    Camera {
      origin: lookfrom,
      lower_left_corner: lookfrom
        - u * focus_dist * half_width
        - v * focus_dist * half_height
        - w * focus_dist,
      horizontal: u * half_width * focus_dist * 2.0,
      vertical: v * half_height * focus_dist * 2.0,
      lens_radius: aperture / 2.0,
      u,
      v,
      w,
    }
  }

  pub fn get_ray(&self, s: f32, t: f32) -> Ray {
    let rd = random_in_unit_disk() * self.lens_radius;
    let offset = self.u * rd.x() + self.v * rd.y();
    Ray {
      A: self.origin + offset,
      B: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
    }
  }
}
