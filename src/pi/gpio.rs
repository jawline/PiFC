#![allow(dead_code)]

use std::fs::File;

pub enum Direction {In, Out}
pub enum State {High, Low}

pub struct Pin {
    port : usize
}

impl Pin {
    
    pub fn new(port : usize) -> Pin {
        export_pin(port);
        Pin{port:port}
    }
    
    fn export_pin(port : usize) -> Result<()> {
        let fmt_port = format!("{}", port);
        File::create("/sys/class/gpio/export").write_all(&fmt_port)
    }
    
    fn get_pin_folder(&self) -> str {
        format!("/sys/class/gpio/gpio{}/", self.port)
    }

    pub fn set_mode(&mut self, direction : Direction) -> bool {
        
        let dir = match direction {
            Direction::In => "in",
            Direction::Out => "out"
        };
        
        let direction_file = File::create(self.get_pin_folder() + "direction");
        
        match direction_file.write_all(&dir) {
            Ok => true,
            Err => false
        }
    }

    pub fn get_mode(&self) -> Option<Direction> {
        let mut direction_str = String::new();
        
        let read_result = File::open(self.get_pin_folder() + "direction").read_to_string(&mut direction_str);
        
        match read_result {
            Ok => match read_result {
                     "in" => Some(Direction::In),
                     "out" => Some(Direction::Out),
                     _ => None
                  },
            Err => None
        }
    }

    pub fn write(&self, state : State) -> bool {
        let current_mode_op = self.get_mode();
        
        match can_write {
            Some(Direction::Out) => true,
            _ => false
        }
        
        if !can_write {
            return false;
        }
        
        let value_file = File::create(self.get_pin_folder() + "value");
        
        let state_str = match state {
            State::High => "1",
            State::Low => "0"
        };

        match value_file.write_all(&state_str) {
            Ok => true,
            Err => false
        }
    }

    pub fn read(&self) -> Option<State> {
        let mut direction_str = String::new();
        let read_result = File::open(self.get_pin_folder() + "value").read_to_string(&mut direction_str);
        match read_result {
            Ok => match read_result {
                     "1" => Some(State::High),
                     "0" => Some(State::Low),
                     _ => None
                  },
            Err => None
        }
    }
}
