pub struct State {
    pub acc: (f64, f64, f64),
    pub gyro: (f64, f64, f64),
    pub gps: (f64, f64)
}

impl State {
    pub fn new() -> State {
        State {
        	acc: (0.0, 0.0, 0.0),
        	gyro: (0.0, 0.0, 0.0),
        	gps: (0.0, 0.0)
        }
    }

    pub fn sample(&mut self) {}
}
