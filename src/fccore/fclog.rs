use std::string::ToString;
use std::vec::Vec;
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
    format!("{:?}: {}", self.time_entered, self.info)
  }
}

pub struct Log {
  pub entries : Vec<LogEntry>
}

impl Log {
  pub fn new() -> LogEntry {
    Log{entries:Vec::new()}
  }

  pub fn add(&mut self, info : &str) {
    self.entries.push(LogEntry::new(info));
  }
}

impl ToString for Log {
  fn to_string(&self) -> String {
    let mut log_data = String::new();
    
    for item in self.entries {
      log_data = log_data + &item.to_string();
    }
    
    log_data
  }
}
