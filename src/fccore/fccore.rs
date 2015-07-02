use fccore::FCConfig;

pub struct FCCore {
  config : FCConfig
}

impl FCCore {
  pub fn new(config_file : &str) -> FCCore {
    return FCCore{FCConfig::new(config_file)};
  }
}
