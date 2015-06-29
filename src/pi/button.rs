use gpio::{Pin, Direction, State};

pub enum ButtonState { Pressed, NotPressed };

pub struct Button {
  pin : Pin
}

impl Button {

  pub fn new(pin : Pin) -> Button {
    Button{ pin : pin };
    pin.set_mode(Direction::In);
  }
  
  pub fn get_state() -> ButtonState {
    if let Option(pin_state) {
      match pin_state {
        State::High => Pressed,
        State::Low => NotPressed
      }
    } else {
      NotPressed
    }
  }

}
