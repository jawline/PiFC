#![allow(dead_code)]

use pi::gpio::{Pin, Direction, State};

pub enum LightState { On, Off }

pub struct Light {
  pin : Pin
}

impl Light {
  pub fn new(pin : Pin) -> Light {
    pin.set_mode(Direction::Out);
    Light{pin : pin}
  }
  
  pub fn set_state(&self, state : LightState) {
    match state {
      On => self.pin_write(State::High),
      Off => self.pin_write(State::Low)
    };
  }

  pub fn enable(&self) {
    self.set_state(On);
  }
  
  pub fn disable(&self) {
    self.set_state(Off);
  }
}
