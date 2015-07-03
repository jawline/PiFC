use std::string::ToString;
use time;

struct LogEntry {
  info : String,
  time_entered : time::Tm
}

impl LogEntry {
  pub fn new(info : &str) -> LogEntry {
    LogEntry{info: info.to_string(), time_entered: time::now()}
  }
}

impl ToString for LogEntry {
  fn to_string(&self) -> String {
    format!("{}: {}", self.time_entered.to_string(), self.info);
  }
}
