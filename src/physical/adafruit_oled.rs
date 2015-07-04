#![allow(dead_code)]

use physical::gpio::{Pin, State, Direction};
use physical::screen::{Screen, PixelScreen};

pub struct AdafruitOled {
  scl : Pin,
  sda : Pin
}

impl AdafruitOled {
  pub fn new(scl : Pin, sda : Pin) -> AdafruitOled {
    AdafruitOled{scl : scl, sda : sda }
  }
}

impl Screen for AdafruitOled {
  fn resolution(&self) -> (usize, usize) {
    (128, 64)
  }
}

impl PixelScreen for AdafruitOled {
  fn set(&self, x : usize, y : usize, color : (u8, u8, u8)) {
    let (r, g, b) = color;
    println!("TODO: AdafruitOled write to screen");
  }
}
