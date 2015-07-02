pub struct FCConfig {
  status_pin : usize,
  arm_switch_pin : usize
}

impl FCConfig {
  pub fn new(base_file : &str) -> FCConfig {
    return FCConfig{};
  }
}
