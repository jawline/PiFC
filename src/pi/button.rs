use gpio::{Pin, Direction, State};

pub struct Button {
  pin : Pin
}

impl Button {

  pub fn new(pin : Pin) -> Button {
    Button{ pin : pin };
  }

}
