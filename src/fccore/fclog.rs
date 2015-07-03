use std::string::ToString;
use time::Timespec;

struct LogEntry {
  info : String,
  time_entered : Timespec
}

impl LogEntry {
  pub fn new(info : &str) -> LogEntry {
    LogEntry{info: info.to_string(), time_entered: time::get_time()}
  }
}

impl ToString for LogEntry {
  fn to_string(&self) -> String {
    format!("{}: {}", self.time_entered, self.info);
  }
}
