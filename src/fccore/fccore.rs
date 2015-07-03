#![allow(dead_code)]

use fccore::fcconfig::FCConfig;
use fccore::fclog::Log;

use pi::gpio::Pin;
use pi::light::{Light, LightState};
use pi::button::{Button, ButtonState};
use pi::polled_button::PolledButton;

pub struct FCCore {
  
  /**
   * Base ARM requirement, safety switch must be switched to on
   */
  armed_switch : bool,
  
  /**
   * Second ARM requirement, a external request must arm the FC
   */
  armed_command : bool,
  
  alive : bool,
  status_led : Light,
  arm_switch : PolledButton,
  config : FCConfig,
  log : Log
}

impl FCCore {
  pub fn new(config_file : &str) -> FCCore {
    let config = FCConfig::load(config_file);
    FCCore {
      armed_switch: false,
      armed_command: false,
      alive : true,
      status_led : Light::new(Pin::new(config.status_pin)),
      arm_switch : PolledButton::new(Pin::new(config.arm_switch_pin)),
      config: config,
      log: Log::new()
    }
  }
  
  pub fn update_sensors(&mut self) {

    //Switch ARM to true if arm switch is pressed
    self.armed_switch = match self.arm_switch.read_state() {
      ButtonState::Pressed => true,
      ButtonState::NotPressed => false
    };
    
    //The ARM from command state is reset to false if the safety is off
    if !self.armed_switch {
      self.armed_command = false;
    }
    
    //Update armed state LED
    self.status_led.set_state(match self.armed() {
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
      self.armed_command = state;
    }
  }

  pub fn config(&self) -> &FCConfig { &self.config }
  
  /**
   * Set the alive flag to false, does not wait for threads to terminate
   */
  pub fn kill(&mut self) { self.alive = false; }

  /**
   * Returns true if the core is still alive, false if it is terminating or terminated
   */
  pub fn alive(&self) -> bool { self.alive }
  
  /**
   * Return the core log
   */
  pub fn log(&self) -> &Log { &self.log }
}
