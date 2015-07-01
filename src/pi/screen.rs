trait Screen {
  fn resolution() -> (usize, usize);
}

trait PixelScreen {
  fn set(x : usize, y : usize, rgb : (u8, u8, u8));
}

trait TextScreen {
  fn set(x : usize, y : usize, text : char);
}
