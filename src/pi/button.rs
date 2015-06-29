use gpio::{Pin, Direction, State};

pub enum ButtonState { Pressed, NotPressed };

pub trait Button {
  pub fn read_state(&self) -> ButtonState;
}
