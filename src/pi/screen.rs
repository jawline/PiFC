pub trait Screen {
  fn resolution() -> (usize, usize);
}

pub trait PixelScreen {
  fn set(x : usize, y : usize, rgb : (u8, u8, u8));
}

pub trait TextScreen {
  fn set(x : usize, y : usize, text : char);
}
