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
    
    let thread_core = core.clone();
    
    spawn(move || {
      FCCore::fccore_thread_loop(thread_core);
    });

    return core;
  }
  
  fn fccore_thread_loop(core : Arc<mut FCCore>) {
    core.armed = false;
  }
}
