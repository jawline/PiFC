pub struct Motor {
    power: usize
}

impl Motor {
    pub fn new() -> Motor {
        Motor{
            power: 0
        }
    }
    
    pub fn current_power(&self) -> usize { self.power }
    pub fn set_power(&mut self, level: usize) { self.power = level; }
}
