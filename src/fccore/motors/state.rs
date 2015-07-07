use fccore::motors::Motor;
use fccore::motors::MotorID;
use std::slice::Iter;

pub struct State {
    motor_1 : Motor,
    motor_2 : Motor,
    motor_3 : Motor,
    motor_4 : Motor
}

impl State {
    pub fn new() -> State {
        State{
            motor_1: Motor::new(),
            motor_2: Motor::new(),
            motor_3: Motor::new(),
            motor_4: Motor::new()
        }
    }
    
    pub fn motor(&self, id: MotorID) -> &Motor {
        match id {
            MotorID::Motor1 => &self.motor_1,
            MotorID::Motor2 => &self.motor_2,
            MotorID::Motor3 => &self.motor_3,
            MotorID::Motor4 => &self.motor_4
        }
    }
    
    pub fn motor_mut(&mut self, id: MotorID) -> &mut Motor {
        match id {
            MotorID::Motor1 => &mut self.motor_1,
            MotorID::Motor2 => &mut self.motor_2,
            MotorID::Motor3 => &mut self.motor_3,
            MotorID::Motor4 => &mut self.motor_4
        }
    }
    
    pub fn iter(&self) -> Iter<&Motor> {
        vec!(self.motor_1, self.motor_2, self.motor_3, self.motor_4).iter()
    }
}
