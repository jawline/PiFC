#![allow(dead_code)]

use fccore::config::Config;
use fccore::configbutton::ConfigButton;
use fccore::log::Log;
use fccore::sensors;
use fccore::motors;
use fccore::motors::MotorID;

use physical::gpio::Pin;
use physical::light::{Light, LightState};

use time;

const TAG : &'static str = "core";
const LOG_DIR : &'static str = "./logs/";

pub struct Core {

    /**
     * Is the core alive
     */
    pub alive : bool,
    
    /**
     * Base ARM requirement, safety switch must be switched to on
     */
    armed_switch : bool,
    
    /**
     * Second ARM requirement, a external request must arm the FC
     */
    armed_command : bool,
    
    /**
     * The armed status LED
     */
    armed_status_led : ConfigLed,
  
    /**
     * ARM safety switch on the device, if set to off position the FC will disable
     */
    armed_safety_switch : ConfigButton,
  
    /**
     * configuration for the core
     */
    config : Config,
  
    /**
     * Core log, stores log messages and timestamps
     */
    log : Log,
    
    /**
     * Telemetry state
     */
    pub sensors : sensors::State,

    /**
     * Motors state
     */
    motors : motors::State
}

impl Core {

    pub fn new(config_file : &str) -> Core {
        let config = Config::load(config_file);
        let mut core = Core {
            armed_switch: false,
            armed_command: false,
            alive : true,
            armed_status_led : ConfigLed::new(&config.armed_led),
            armed_safety_switch : ConfigButton::new(&config.arm_switch),
            motors: motors::State::new(&config.motors),
            log: Log::new(&format!("{}log{}", LOG_DIR, time::now().to_timespec().sec)),
            sensors: sensors::State::new(),
            config: config
        };
        core.armed_changed();
        core
    }
    
    /**
     * Check the state of sensors and react to any changes
     */
    pub fn update_sensors(&mut self) {
    
        //Read from the physical safety
        let safety_state = self.armed_safety_switch.read_state();

        if safety_state && !self.armed_switch {
            self.log_mut().add(TAG, "physical safety switched to armed");
            self.armed_switch = true;
        } else if !safety_state && self.armed_switch {
            self.log_mut().add(TAG, "physical safety switched to disarmed");
            self.armed_switch = false;
        }
        
        //The ARM from command state is reset to false if the safety is off
        if !self.armed_switch && self.armed_command {
            self.log_mut().add(TAG, "set core armed_command to false as switch is false");
            self.armed_command = false;
            self.armed_changed();
        }
        
        //Update armed state LED
        self.armed_status_led.set_state(match self.armed() {
            true => LightState::On,
            false => LightState::Off
        });
        
        //Take gyroscope and accelerometer readings
        self.sensors.sample();
        
        //Log any accelerometer data
        let (acc_x, acc_y, acc_z) = self.sensors.acc;
        if acc_x + acc_y + acc_z != 0.0 {
            self.log_mut().add(TAG, "accelerometer reading non 0");
        }
        
        //Log any gyro data
        let (gyr_x, gyr_y, gyr_z) = self.sensors.gyro;
        if gyr_x + gyr_y + gyr_z != 0.0 {
            self.log_mut().add(TAG, "gyro reading non 0");
        }
    }

    fn armed_changed(&mut self) {
        self.log.add(TAG, "armed_changed triggered");
        if self.armed() {
            self.enable_all_motors();
        } else {
            self.disable_all_motors();
        }
    }
    
    fn enable_all_motors(&mut self) {
        self.log.add(TAG, "enable all motors");
        for item in self.motors.iter_mut() {
            item.enable(&mut self.log);
        }
    }

    fn disable_all_motors(&mut self) {
        self.log.add(TAG, "disable all motors");
        for item in self.motors.iter_mut() {
            item.disable(&mut self.log);
        }
    }
  
    /**
     * true if the device is fully armed
     */
    pub fn armed(&self) -> bool { self.armed_switch && self.armed_command }
  
    /**
     * true if an external arm is set
     */
    pub fn armed_cmd(&self) -> bool { self.armed_command }
    
    /**
     * true if the physical safety arm switch is armed
     */
    pub fn armed_switch(&self) -> bool { self.armed_switch }
  
    /**
     * Set the command ARM state
     * If the physical ARM button is off this will do nothing
     */
    pub fn set_armed_command(&mut self, state : bool) {
        if self.armed_switch {
            self.log_mut().add(TAG, &format!("ARM command request to set to {} handled at core", state));
            self.armed_command = state;
        } else {
            self.log_mut().add(TAG, "ARM command request ignored as armed_switch is disabled");
        }
        self.armed_changed();
    }
    
    /**
     * Return an immutable ref to the motors state
     */
    pub fn motors(&self) -> &motors::State { &self.motors }
    
    /**
     * Set a motors power level
     */
    pub fn set_motor_power(&mut self, motor: MotorID, level: usize) {
        self.motors.motor_mut(motor).set_power(level, &mut self.log);
    }

    /**
     * Get the core config struct
     */
    pub fn config(&self) -> &Config { &self.config }
    
    /**
     * Return the core log
     */
    pub fn log(&self) -> &Log { &self.log }
    
    /**
     * Return the core log as mutable
     */
     pub fn log_mut(&mut self) -> &mut Log { &mut self.log }
}
