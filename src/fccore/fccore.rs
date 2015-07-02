use fccore::fcconfig::FCConfig;
use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex};

pub struct FCCore {
  armed : bool,
  config : FCConfig
}

impl FCCore {
  pub fn new(config_file : &str) -> Arc<FCCore> {
  
    let core = Arc::new(FCCore{
      armed: false,
      config: FCConfig::new(config_file)
    });
    
    spawn(move || {
      fccore_thread_loop(core);
    });

    return core;
  }
  
  fn fccore_thread_loop(core : Arc<FCCore>) {
  }
}
