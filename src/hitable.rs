use super::{
  vec3::Vec3,
  ray::Ray
};

pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
}

impl HitRecord {
  pub fn new() -> HitRecord {
    HitRecord {
      t: 0.0,
      p: Vec3::new(0.0, 0.0, 0.0),
      normal: Vec3::new(0.0, 0.0, 0.0)
    }
  }
}

pub trait Hitable {
  fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub type HitableList = Vec<Box<Hitable>>;

impl Hitable for HitableList {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::new();
    let mut hit_anything = false;
    let mut closet_so_far = tmax;
    for i in 0..self.len() {
      if self[i].hit(r, tmin, closet_so_far, &mut temp_rec) {
        hit_anything = true;
        closet_so_far = temp_rec.t;
        rec.t = temp_rec.t;
        rec.p = temp_rec.p;
        rec.normal = temp_rec.normal;
      }
    }
    hit_anything
  }
}

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(cen: Vec3, r: f32) -> Self {
    Sphere {
      center: cen, 
      radius: r
    }
  }
}

impl Hitable for Sphere {
  fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitRecord) -> bool {
    let oc = *r.origin() - self.center;
    let a = r.direction().dot(r.direction());
    let b = oc.dot(r.direction());
    let c = oc.dot(&oc) - self.radius.powi(2);
    let discriminant = b.powi(2) - a * c;
    if discriminant > 0.0 {
      let mut temp = (-b - (b.powi(2) - a * c).sqrt()) / a;
      if temp < tmax && temp > tmin {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
      temp = (-b + (b.powi(2) - a * c).sqrt()) / a;
      if temp < tmax && temp > tmin {
        rec.t = temp;
        rec.p = r.point_at_parameter(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
      }
    }
    false
  }
}