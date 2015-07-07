use fccore::motors::Motor;
use fccore::motors::MotorID;
use std::slice::Iter;
use std::vec::Vec;

pub struct State {
    motors : Vec<Motor>
}

impl State {
    pub fn new() -> State {
        State{
            motors: vec!(Motor::new(), Motor::new(), Motor::new(), Motor::new())
        }
    }
    
    pub fn motor(&self, id: MotorID) -> &Motor {
        match id {
            MotorID::Motor1 => &self.motors[0],
            MotorID::Motor2 => &self.motors[1],
            MotorID::Motor3 => &self.motors[2],
            MotorID::Motor4 => &self.motors[3]
        }
    }
    
    pub fn motor_mut(&mut self, id: MotorID) -> &mut Motor {
        match id {
            MotorID::Motor1 => &mut self.motors[0],
            MotorID::Motor2 => &mut self.motors[1],
            MotorID::Motor3 => &mut self.motors[2],
            MotorID::Motor4 => &mut self.motors[3]
        }
    }
    
    pub fn as_mut_vec(&mut self) -> Iter<Motor> {
        self.motors.iter()
    }
}
