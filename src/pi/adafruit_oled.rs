use pi::gpio::{Pin, State, Direction};
use pi::screen::{Screen, PixelScreen};

pub struct AdafruitOled {
  scl : Pin,
  sda : Pin
}

impl Screen for AdafruitOled {
  fn resolution() -> (usize, usize) {
    (128, 64)
  }
}

impl PixelScreen for AdafruitOled {
  fn set(x : usize, y : usize, (r : u8, g : u8, b : u8)) {
  }
}
