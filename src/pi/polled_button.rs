use button::{Button, ButtonState};
use gpio::{Pin, Direction, State};

pub struct PolledButton {
  pin : Pin
}

impl PolledButton {
  pub fn new(pin : mut Pin) -> PolledButton {
    pin.set_mode(Direction::In);
    PolledButton{pin : pin}
  }
}

impl Button for PolledButton {
  pub fn read_state(&self) -> ButtonState {
    if let Option(state) = self.pin.read() {
      match state {
        State::High => ButtonState::Pressed,
        State::Low => ButtonState::NotPressed
      }
    } else {
      ButtonState::NotPressed
    }
  }
}
