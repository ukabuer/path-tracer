extern crate rand;

use super::{ray::Ray, vec3::Vec3, material::Material};
use std::rc::Rc;

pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub mat: Rc<Material>,
}

pub trait Hitable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub type HitableList = Vec<Box<Hitable>>;

impl Hitable for HitableList {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let mut closet_so_far = tmax;
    let mut result: Option<HitRecord> = None;
    for i in 0..self.len() {
      match self[i].hit(r, tmin, closet_so_far) {
        Some(record) => {
          closet_so_far = record.t;
          result = Some(record);
        }
        _ => (),
      }
    }
    result
  }
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
  pub mat: Rc<Material>,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32, mat: Rc<Material>) -> Self {
    Sphere {
      center,
      radius,
      mat,
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
    let oc = *r.origin() - self.center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction());
    let c = oc.dot(&oc) - self.radius.powi(2);
    let discriminant = b.powi(2) - a * c;
    if discriminant > 0.0 {
      let mut temp = (-b - (b.powi(2) - a * c).sqrt()) / a;
      if temp < tmax && temp > tmin {
        let p = r.point_at_parameter(temp);
        return Some(HitRecord {
          t: temp,
          p,
          normal: (p - self.center) / self.radius,
          mat: Rc::clone(&self.mat.clone()),
        });
      }
      temp = (-b + (b.powi(2) - a * c).sqrt()) / a;
      if temp < tmax && temp > tmin {
        let p = r.point_at_parameter(temp);
        return Some(HitRecord {
          t: temp,
          p,
          normal: (p - self.center) / self.radius,
          mat: Rc::clone(&self.mat.clone()),
        });
      }
    }
    None
  }
}

