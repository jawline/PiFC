use fccore::log::Log;
use fccore::config;
use std::string::String;

const TAG : &'static str = "motor";

pub struct Motor {
    name: String,
    power: usize,
    enabled: bool
}

impl Motor {
    pub fn new(config: &config::Motor) -> Motor {
        Motor{
            name: config.name.clone(),
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
        if self.enabled {
            self.enabled = false;
            self.power = 0;
            log.add(TAG, "disabled motor");
        }
    }

    pub fn enable(&mut self, log: &mut Log) {
        self.enabled = true;
        log.add(TAG, "enabled motor");
    }
}
