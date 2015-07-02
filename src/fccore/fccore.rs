use fccore::fcconfig::FCConfig;
use std::thread::{spawn, JoinHandle};

pub struct FCCore {
  armed : bool,
  config : FCConfig
}

impl FCCore {
  pub fn new(config_file : &str) -> &mut FCCore {
    
    let mut core = FCCore{
      armed: false,
      config: FCConfig::new(config_file)
    };
    
    spawn(|| {
      FCCore::fccore_thread_loop(&mut core); 
    });
    
    return core;
  }
  
  fn fccore_thread_loop(core : &mut FCCore) {
  }
}
