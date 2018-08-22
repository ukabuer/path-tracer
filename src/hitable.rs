extern crate rand;

use super::{ray::Ray, vec3::Vec3};
use rand::random;
use std::rc::Rc;

fn random_in_unit_sphere() -> Vec3 {
  let mut p;
  loop {
    p = Vec3::new(
      2.0 * random::<f32>() - 1.0,
      2.0 * random::<f32>() - 1.0,
      2.0 * random::<f32>() - 1.0,
    );
    if p.dot(&p) < 1.0 {
      return p;
    }
  }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
  v - n * v.dot(&n) * 2.0
}

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
    let closet_so_far = tmax;
    let mut result: Option<HitRecord> = None;
    for i in 0..self.len() {
      result = match self[i].hit(r, tmin, closet_so_far) {
        None => None,
        Some(record) => Some(record),
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

pub trait Material {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
  pub albedo: Vec3,
}

impl Lambertian {
  pub fn new(albedo: Vec3) -> Lambertian {
    Lambertian { albedo }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
    let target = rec.p + rec.normal + random_in_unit_sphere();
    Some((Ray::new(rec.p, target - rec.p), self.albedo))
  }
}

pub struct Metal {
  pub albedo: Vec3,
}

impl Metal {
  pub fn new(albedo: Vec3) -> Metal {
    Metal { albedo }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
    let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
    let scattered = Ray::new(rec.p, reflected);
    if scattered.direction().dot(&rec.normal) > 0.0 {
      Some((scattered, self.albedo))
    } else {
      None
    }
  }
}
