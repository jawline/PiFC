use pi::screen::{Screen, PixelScreen};

pub struct AdafruitOled {
  scl : Pin,
  sda : Pin
}

impl Screen for AdafruitOled {
}

impl PixelScreen for AdafruitOled {
}
