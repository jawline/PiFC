#![allow(dead_code)]

use std::fs::File;
use std::io::{Write, Read};

pub enum Direction {In, Out}
pub enum State {High, Low}

pub struct Pin {
    port : usize
}

impl Pin {
    
    pub fn new(port : usize) -> Pin {
        Pin::export_pin(port);
        Pin{port:port}
    }
    
    fn export_pin(port : usize) -> bool {
        let fmt_port = &format!("{}", port);
        match File::create("/sys/class/gpio/export") {
            Ok(_) => match file.write_all(fmt_port.as_bytes()) {
                        Ok(_) => true
                        Err(_) => false
                     },
            Err(_) => false
        }
    }
    
    fn get_pin_folder(&self) -> String {
        format!("/sys/class/gpio/gpio{}/", self.port)
    }

    pub fn set_mode(&mut self, direction : Direction) -> bool {
        
        let direction_file_res = File::create(self.get_pin_folder() + "direction");
        
        if let Err(_) = direction_file_res {
            return false;
        }

        let direction_file = direction_file_res.unwrap();        
        
        let dir = match direction {
            Direction::In => "in",
            Direction::Out => "out"
        };

        match direction_file.write_all(dir.as_bytes()) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn get_mode(&self) -> Option<Direction> {
        let mut direction_str = String::new();
        
        let file_open_res = File::open(self.get_pin_folder() + "direction");
        
        if let Err(err_msg) = file_open_res {
            return None;
        }
        
        let read_result = file_open_res.unwrap().read_to_string(&mut direction_str);
        let direction : &str = &direction_str;
        
        match read_result {
            Ok(_) => match direction {
                     "in" => Some(Direction::In),
                     "out" => Some(Direction::Out),
                     _ => None
                  },
            Err(_) => None
        }
    }

    pub fn write(&self, state : State) -> bool {
        let current_mode_op = self.get_mode();
        
        let can_write = match current_mode_op {
            Some(Direction::Out) => true,
            _ => false
        };
        
        if !can_write {
            return false;
        }
        
        let value_file_create_res = File::create(self.get_pin_folder() + "value");
        
        if let Err(_) = value_file_create_res {
            return false;
        }
        
        let value_file = value_file_create_res.unwrap();
        
        let state_str = match state {
            State::High => "1",
            State::Low => "0"
        };

        match value_file.write_all(state_str.as_bytes()) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn read(&self) -> Option<State> {
        let file_open_res = File::open(self.get_pin_folder() + "value");
        
        if let Err(_) = file_open_res {
            return None;
        }
        
        let mut pin_value_str = String::new();
        let read_result = file_open_res.unwrap().read_to_string(&mut pin_value_str);
        let pin_value : &str = &pin_value_str;
        
        match read_result {
            Ok(_) => match pin_value {
                     "1" => Some(State::High),
                     "0" => Some(State::Low),
                     _ => None
                  },
            Err(_) => None
        }
    }
}
