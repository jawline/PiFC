pub struct State {
    pub acc : (f64, f64, f64),
    pub gyro : (f64, f64, f64)
}

impl State {
    pub fn new() -> State {
        State{acc: (0, 0, 0), gyr: (0, 0, 0)}
    }

    pub fn sample(&mut self) {}

    pub fn clear(&mut self) {
        self.acc = (0, 0, 0);
        self.gyro = (0, 0, 0);
    }
}
