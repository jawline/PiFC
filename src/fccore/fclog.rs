use std::string::ToString;
use std::vec::Vec;
use time;

pub struct LogEntry {
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
    format!("{}: {}", self.time_entered.rfc822(), self.info)
  }
}

pub struct Log {
  entries : Vec<LogEntry>
}

impl Log {
  pub fn new() -> Log {
    Log{entries:Vec::new()}
  }

  pub fn add(&mut self, info : &str) {
    let entry = LogEntry::new(info);
    println!("Add {} to log", entry.to_string());
    self.entries.push(entry);
  }
}

impl ToString for Log {
  fn to_string(&self) -> String {
    let mut log_data = String::new();
    
    for item in &self.entries {
      log_data = log_data + &format!("{}\n", item.to_string());
    }
    
    log_data
  }
}
