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
  fn set(x : usize, y : usize, color : (u8, u8, u8)) {
    let (r, g, b) = color;
    println!("TODO: AdafruitOled write to screen");
  }
}
