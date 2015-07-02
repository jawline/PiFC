use fccore::fcconfig::FCConfig;
use std::thread::{spawn, JoinHandle};

pub struct FCCore {
  armed : bool,
  config : FCConfig
}

impl<'a> FCCore<'a> {
  pub fn new(config_file : &str) -> FCCore<'a> {
  
    let mut core = FCCore{
      armed: false,
      config: FCConfig::new(config_file)
    };

    core.start_thread();
    return core;
  }
  
  fn start_thread<T: FnMut() + 'a>(&mut self) {
    spawn(|| {
      self.fccore_thread_loop();
    });    
  }
  
  fn fccore_thread_loop(&mut self) {
    self.armed = false;
  }
}
