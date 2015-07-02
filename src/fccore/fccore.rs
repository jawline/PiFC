use fccore::fcconfig::FCConfig;
use std::thread::{spawn, JoinHandle};
use std::sync::{Mutex};

pub struct FCCore {
  armed : bool,
  config : FCConfig
}

impl FCCore {
  pub fn new(config_file : &str) -> Mutex<FCCore> {
  
    let core = Mutex::new(FCCore{
      armed: false,
      config: FCConfig::new(config_file)
    });
    
    spawn(move || {
      FCCore::fccore_thread_loop(core);
    });

    return core;
  }
  
  fn fccore_thread_loop(core : Mutex<FCCore>) {
    core.lock().unwrap().armed = false;
  }
}
