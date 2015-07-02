use fccore::fcconfig::FCConfig;
use pi::gpio::Pin;
use pi::light::{Light, LightState};
use pi::button::{Button, ButtonState};
use pi::polled_button::PolledButton;
use std::thread::{spawn, JoinHandle, sleep_ms};
use std::sync::{Arc, Mutex};

pub struct FCCore {
  pub armed : bool,
  alive : bool,
  status_led : Light,
  arm_switch : PolledButton,
  config : FCConfig,
  join_handle : Option<JoinHandle>
}

impl FCCore {
  pub fn new(config_file : &str) -> Arc<Mutex<FCCore>> {

    let config = FCConfig::load(config_file);
  
    let core = Arc::new(Mutex::new(FCCore{
      armed: false,
      alive : true,
      status_led : Light::new(Pin::new(config.status_pin)),
      arm_switch : PolledButton::new(Pin::new(config.arm_switch_pin)),
      config: config,
      join_handle: None
    }));
    
    let thread_core = core.clone();
    
    core.lock().unwrap().join_handle = spawn(move || {
      FCCore::fccore_thread_loop(thread_core);
    });

    return core;
  }
  
  fn update_sensors(&mut self) {
    //Switch ARM to true if arm switch is pressed
    self.armed = match self.arm_switch.read_state() {
      ButtonState::Pressed => true,
      ButtonState::NotPressed => false
    };
    
    //Update armed state LED
    self.status_led.set_state(match self.armed {
      true => LightState::On,
      false => LightState::Off
    });
  }
  
  fn fccore_thread_loop(core_ref : Arc<Mutex<FCCore>>) {
    while alive {
      sleep_ms(50);
      core_ref.lock().unwrap().update_sensors();
    }
  }
  
  pub fn kill(&mut self) {
    self.alive = false;
    if let Some(handle) = self.join_handle {
      handle.join();
    }
  }
}
