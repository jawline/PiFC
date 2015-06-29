#![allow(dead_code)]

use std::fs::File;

pub enum Direction {In, Out}
pub enum State {High, Low}

pub struct Pin {
    port : usize
}

impl Pin {
    
    pub fn new(port : usize) -> Pin {
        Pin{port:port}
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
        match self.get_mode() {
            Direction::In => false,
            Direction::Out => true
        }
    }

    pub fn read(&self) -> Option<State> {
        None
    }
}
