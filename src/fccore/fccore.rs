#![allow(dead_code)]

use fccore::fcconfig::FCConfig;
use fccore::fclog::Log;

use physical::gpio::Pin;
use physical::light::{Light, LightState};
use physical::button::{Button, ButtonState};
use physical::polled_button::PolledButton;

const TAG : &'static str = "core";
const LOG_FILE : &'static str = "./core.log";

pub struct FCCore {
  
  /**
   * Is the core alive
   */
  alive : bool,
  
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
  armed_status_led : Light,

  /**
   * ARM safety switch on the device, if set to off position the FC will disable
   */
  armed_safety_switch : PolledButton,

  /**
   * configuration for the core
   */
  config : FCConfig,

  /**
   * Core log, stores log messages and timestamps
   */
  log : Log
}

impl FCCore {
  pub fn new(config_file : &str) -> FCCore {
    let config = FCConfig::load(config_file);
    FCCore {
      armed_switch: false,
      armed_command: false,
      alive : true,
      armed_status_led : Light::new(Pin::new(config.status_pin)),
      armed_safety_switch : PolledButton::new(Pin::new(config.arm_switch_pin)),
      config: config,
      log: Log::new(LOG_FILE)
    }
  }
  
  /**
   * Check the state of sensors and react to any changes
   */
  pub fn update_sensors(&mut self) {

    //Switch ARM to true if arm switch is pressed
    self.armed_switch = match self.armed_safety_switch.read_state() {
      ButtonState::Pressed => true,
      ButtonState::NotPressed => false
    };
    
    //The ARM from command state is reset to false if the safety is off
    if !self.armed_switch && self.armed_command {
      self.log_mut().add(TAG, "set core armed_command to false as switch is false");
      self.armed_command = false;
    }
    
    //Update armed state LED
    self.armed_status_led.set_state(match self.armed() {
      true => LightState::On,
      false => LightState::Off
    });
  }
  
  pub fn armed(&self) -> bool { self.armed_switch && self.armed_command }
  pub fn armed_cmd(&self) -> bool { self.armed_command }
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
  }

  /**
   * Get the core config struct
   */
  pub fn config(&self) -> &FCConfig { &self.config }
  
  /**
   * Set the alive flag to false, does not wait for threads to terminate
   */
  pub fn kill(&mut self) {
    self.alive = false;
  }

  /**
   * Returns true if the core is still alive, false if it is terminating or terminated
   */
  pub fn alive(&self) -> bool { self.alive }
  
  /**
   * Return the core log
   */
  pub fn log(&self) -> &Log { &self.log }
  
  /**
   * Return the core log as mutable
   */
   pub fn log_mut(&mut self) -> &mut Log { &mut self.log }
}
