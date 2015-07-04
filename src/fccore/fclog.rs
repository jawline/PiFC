use std::string::ToString;
use std::vec::Vec;
use time;

struct LogEntry {
  tag  : String,
  info : String,
  time_entered : time::Tm
}

impl LogEntry {
  pub fn new(tag : &str, info : &str) -> LogEntry {
    LogEntry{tag: tag.to_string(), info: info.to_string(), time_entered: time::now()}
  }
}

impl ToString for LogEntry {
  fn to_string(&self) -> String {
    format!("{}- {}: {}", self.time_entered.rfc822(), self.tag, self.info)
  }
}

pub struct Log {
  entries : Vec<LogEntry>
}

impl Log {
  pub fn new() -> Log {
    Log{entries:Vec::new()}
  }

  pub fn add(&mut self, tag : &str, info : &str) {
    let entry = LogEntry::new(tag, info);
    println!("Add {} to log", entry.to_string());
    self.entries.push(entry);
  }
}

impl ToString for Log {
  fn to_string(&self) -> String {
    if self.entries.len() == 0 {
      return format!("Log Empty");
    } else {
      let mut log_data = String::new();
      
      for item in &self.entries {
        log_data = log_data + &format!("{}\n", item.to_string());
      }
      
      return log_data;
    }
  }
}
