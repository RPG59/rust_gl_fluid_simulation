#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2 {
  pub x: f32,
  pub y: f32,
}

impl Vec2 {
  pub fn sub(&self, v: &Vec2) -> Self {
    Vec2 {
      x: self.x - v.x,
      y: self.y - v.y
    }
  }
}
