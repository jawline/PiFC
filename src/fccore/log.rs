use std::string::ToString;
use std::fs::File;
use std::vec::Vec;
use std::io::Write;
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
    entries : Vec<LogEntry>,
    out_file : File
}

impl Log {
    pub fn new(log_file : &str) -> Log {
       Log{entries:Vec::new(), out_file: File::create(log_file).unwrap()}
    }
  
    pub fn add(&mut self, tag : &str, info : &str) {
        let entry = LogEntry::new(tag, info);
        println!("{}", entry.to_string());

        let log_string = format!("{}\n", entry.to_string());

        if let Err(_) = self.out_file.write_all(log_string.as_bytes()) {
            println!("Could not write to log file");
        }

        self.entries.push(entry);
    }

    fn generate_log_string(&self) -> String {
        self.entries.map(|&item| item.to_string()).fold(|curr, &next| curr + next );
    }
}

impl ToString for Log {
    fn to_string(&self) -> String {
        match self.entries.is_empty() {
            true => format!{"Log Empty"},
            false => self.generate_log_string()
        }
    }
}
