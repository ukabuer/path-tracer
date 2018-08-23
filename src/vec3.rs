use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
  e: [f32; 3],
}

impl Div<f32> for Vec3 {
  type Output = Vec3;
  fn div(self, t: f32) -> Vec3 {
    Vec3 {
      e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
    }
  }
}

impl Vec3 {
  pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { e: [x, y, z] }
  }

  pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
  }

  pub fn x(&self) -> f32 {
    self.e[0]
  }

  pub fn y(&self) -> f32 {
    self.e[1]
  }

  pub fn length(&self) -> f32 {
    (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
  }

  pub fn dot(&self, other: &Vec3) -> f32 {
    self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
  }

  pub fn cross(&self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self[1] * other[2] - other[1] * self[2],
        self[2] * other[0] - other[2] * self[0],
        self[0] * other[1] - other[0] * self[1],
      ],
    }
  }
}

impl Add for Vec3 {
  type Output = Vec3;

  fn add(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] + other.e[0],
        self.e[1] + other.e[1],
        self.e[2] + other.e[2],
      ],
    }
  }
}

impl Sub for Vec3 {
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] - other.e[0],
        self.e[1] - other.e[1],
        self.e[2] - other.e[2],
      ],
    }
  }
}

impl Mul for Vec3 {
  type Output = Vec3;
  fn mul(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] * other.e[0],
        self.e[1] * other.e[1],
        self.e[2] * other.e[2],
      ],
    }
  }
}

impl Mul<f32> for Vec3 {
  type Output = Vec3;
  fn mul(self, t: f32) -> Vec3 {
    Vec3 {
      e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
    }
  }
}

impl Div for Vec3 {
  type Output = Vec3;
  fn div(self, other: Vec3) -> Vec3 {
    Vec3 {
      e: [
        self.e[0] / other.e[0],
        self.e[1] / other.e[1],
        self.e[2] / other.e[2],
      ],
    }
  }
}

impl Index<usize> for Vec3 {
  type Output = f32;
  fn index(&self, index: usize) -> &f32 {
    &self.e[index]
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut f32 {
    &mut self.e[index]
  }
}

impl Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Vec3 {
    Vec3 {
      e: [-self.e[0], -self.e[1], -self.e[2]],
    }
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, other: Vec3) {
    self.e[0] += other.e[0];
    self.e[1] += other.e[1];
    self.e[2] += other.e[2];
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, other: Vec3) {
    self.e[0] -= other.e[0];
    self.e[1] -= other.e[1];
    self.e[2] -= other.e[2];
  }
}

impl MulAssign<f32> for Vec3 {
  fn mul_assign(&mut self, t: f32) {
    self.e[0] *= t;
    self.e[1] *= t;
    self.e[2] *= t;
  }
}

impl DivAssign<f32> for Vec3 {
  fn div_assign(&mut self, t: f32) {
    self.e[0] /= t;
    self.e[1] /= t;
    self.e[2] /= t;
  }
}
