use fccore::log::Log;

const TAG : &'static str = "motor";

pub struct Motor {
    power: usize,
    enabled: bool
}

impl Motor {
    pub fn new() -> Motor {
        Motor{
            power: 0,
            enabled: false
        }
    }
    
    pub fn current_power(&self) -> usize { self.power }
    pub fn set_power(&mut self, level: usize, log: &mut Log) {
        if self.enabled {
            log.add(TAG, &format!("Motor set power level to {}", level));
            self.power = level;
        } else {
            log.add(TAG, &format!("Motor disabled - power request ignored, set level to 0"));
            self.power = 0;
        }
    }

    pub fn disable(&mut self, log: &mut Log) {
        self.enabled = false;
        self.power = 0;
        log.add(TAG, &format!("disabled motor"));
    }

    pub fn enable(&mut self, log: &mut Log) {
        self.enabled = true;
        log.add(TAG, &format("enabled motor"));
    }
}
