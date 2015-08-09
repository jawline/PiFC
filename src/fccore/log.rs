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

pub enum Lines {
    All,
    Limit(usize)
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

        if self.out_file.write_all(log_string.as_bytes()).is_err() {
            println!("Could not write to log file");
        }

        self.entries.push(entry);
    }

    pub fn to_string_lines_max(&self, lines: Lines) -> String {

        let to_skip = match lines {
            Lines::Limit(size) if self.entries.len() > size => self.entries.len() - size,
            _ => 0
        };

        match self.entries.is_empty() {
            true => format!{"Log Empty"},
            false => self.entries.iter().skip(to_skip).fold(String::new(), |curr, next| curr + "\n" + &next.to_string())
        }
    }
}

impl ToString for Log {
    fn to_string(&self) -> String {
        self.to_string_lines_max(Lines::All)
    }
}
