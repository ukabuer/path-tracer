extern crate rand;

use super::{hitable::HitRecord, ray::Ray, vec3::Vec3};
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

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
  let dt = v.dot(&n);
  let discriminiant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
  if discriminiant > 0.0 {
    Some((v - n * dt) * ni_over_nt - n * discriminiant.sqrt())
  } else {
    None
  }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0 * r0;
  r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
  pub fuzz: f32,
}

impl Metal {
  pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
    Metal { albedo, fuzz }
  }
}

impl Material for Metal {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
    let reflected = reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
    let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
    if scattered.direction().dot(&rec.normal) > 0.0 {
      Some((scattered, self.albedo))
    } else {
      None
    }
  }
}

pub struct Dielectrics {
  pub ref_idx: f32,
}

impl Dielectrics {
  pub fn new(ref_idx: f32) -> Dielectrics {
    Dielectrics { ref_idx }
  }
}

impl Material for Dielectrics {
  fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
    let reflected = reflect(*r_in.direction(), rec.normal);
    let attenuation = Vec3::new(1.0, 1.0, 1.0);
    let dot = r_in.direction().dot(&rec.normal);
    let (outward_normal, ni_over_nt, cosine) = if dot > 0.0 {
      (
        -rec.normal,
        self.ref_idx,
        self.ref_idx * dot / r_in.direction().length(),
      )
    } else {
      (
        rec.normal,
        1.0 / self.ref_idx,
        -dot / r_in.direction().length(),
      )
    };
    match refract(*r_in.direction(), outward_normal, ni_over_nt) {
      Some(refracted) => {
        let reflect_prob = schlick(cosine, self.ref_idx);
        if random::<f32>() < reflect_prob {
          Some((Ray::new(rec.p, reflected), attenuation))
        } else {
          Some((Ray::new(rec.p, refracted), attenuation))
        }
      },
      None => Some((Ray::new(rec.p, reflected), attenuation)),
    }
  }
}
