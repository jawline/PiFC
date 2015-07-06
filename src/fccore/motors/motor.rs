use fccore::fclog::Log;

const TAG : &'static str = "motor";

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
    pub fn set_power(&mut self, level: usize, log: &mut Log) {
        log.add(TAG, &format!("Motor set power level to {}", level));
        self.power = level;
    }
}
