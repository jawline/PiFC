#![allow(dead_code)]
use gpio::{Pin, Direction, State};

pub enum LightState { On, Off }

pub struct Light {
    pin : Pin
}

impl Light {
    pub fn new(pin : Pin) -> Light {
        pin.set_mode(Direction::Out);
        Light{pin: pin}
    }
    
    pub fn set_state(&self, state : LightState) {
        match state {
            LightState::On => self.pin.write(State::High),
            LightState::Off => self.pin.write(State::Low)
        };
    }
  
    pub fn enable(&self) {
        self.set_state(LightState::On);
    }
    
    pub fn disable(&self) {
        self.set_state(LightState::Off);
    }
}
