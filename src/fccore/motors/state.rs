use fccore::motors::Motor;
use fccore::motors::MotorID;

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
    
    pub fn moter(&self, id: MotorID) -> &Motor {
        match id {
            Motor1 => &self.motor_1,
            Motor2 => &self.motor_2,
            Motor3 => &self.motor_3,
            Motor4 => &self.motor_4
        }
    }
    
    pub fn moter_mut(&mut self, id: MotorID) -> &mut Motor {
        match id {
            Motor1 => &mut self.motor_1,
            Motor2 => &mut self.motor_2,
            Motor3 => &mut self.motor_3,
            Motor4 => &mut self.motor_4
        }
    }
}
