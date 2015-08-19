use fccore::log::Log;
use fccore::config;
use std::string::String;

const TAG : &'static str = "motor";

pub struct Motor {
    pub name: String,
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
            log.add(TAG, &format!("motor named {} set power level to {}", self.name, level));
            self.power = level;
        } else {
            log.add(TAG, &format!("motor named {} disabled - power request ignored, set level to 0", self.name));
            self.power = 0;
        }
    }

    pub fn disable(&mut self, log: &mut Log) {
        if self.enabled {
            self.enabled = false;
            self.power = 0;
            log.add(TAG, &format!("disabled motor named {}", self.name));
        }
    }

    pub fn enable(&mut self, log: &mut Log) {
        self.enabled = true;
        log.add(TAG, &format!("enabled motor named {}", self.name));
    }

    pub fn enabled(&self) -> bool { self.enabled }
}
