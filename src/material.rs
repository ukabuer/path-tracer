extern crate rand;

use super::{ray::Ray, vec3::Vec3, hitable::HitRecord};
use rand::random;

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
