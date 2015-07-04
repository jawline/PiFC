pub trait Screen {
  fn resolution(&self) -> (usize, usize);
}

pub trait PixelScreen {
  fn set(&self, x : usize, y : usize, rgb : (u8, u8, u8));
}

pub trait TextScreen {
  fn set(&self, x : usize, y : usize, text : char);
}
