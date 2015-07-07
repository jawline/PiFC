use fccore::motors::Motor;
use fccore::motors::MotorID;
use fccore::config;
use std::slice::{Iter, IterMut};
use std::vec::Vec;

pub struct State {
    motors : Vec<Motor>
}

impl State {
    pub fn new(configs: &[4; config::Motor]) -> State {
        State{
            motors: vec!(Motor::new(configs[0]), Motor::new(configs[1]), Motor::new(configs[2]), Motor::new(configs[3]))
        }
    }
    
    fn motor_index(id: MotorID) -> usize {
        match id {
            MotorID::FrontLeft => 0,
            MotorID::FrontRight => 1,
            MotorID::BackLeft => 2,
            MotorID::BackRight => 3
        }
    }
    
    pub fn motor(&self, id: MotorID) -> &Motor {
        &self.motors[State::motor_index(id)]
    }
    
    pub fn motor_mut(&mut self, id: MotorID) -> &mut Motor {
        &mut self.motors[State::motor_index(id)]
    }
    
    pub fn iter(&self) -> Iter<Motor> {
        self.motors.iter()
    }
    
    pub fn iter_mut(&mut self) -> IterMut<Motor> {
        self.motors.iter_mut()
    }
}
