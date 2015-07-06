use fccore::motors::Motor;

pub struct State {
    pub motor_1 : Motor,
    pub motor_2 : Motor,
    pub motor_3 : Motor,
    pub motor_4 : Motor
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
}
