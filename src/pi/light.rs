use gpio::{Pin, Direction, State};

pub struct Light {
  pin : Pin
}

impl Light {

  pub fn new(pin : Pin) -> Light {
    pin.set_mode(Direction::Out);
    Light{pin : pin};
  }

  pub fn enable(&self) {
    pin.write(State::High);
  }
  
  pub fn disable(&self) {
    pin.write(State::Low);
  }
}
