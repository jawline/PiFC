use pi::gpio::{Pin, Direction, State};

pub enum ButtonState { Pressed, NotPressed }

pub trait Button {
  fn read_state(&self) -> ButtonState;
}
