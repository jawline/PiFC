#![allow(dead_code)]

use std::fs::File;
use std::io::{Write, Read};

pub enum Direction {In, Out}
pub enum State {High, Low}

pub struct Pin {
    port: usize
}

impl Pin {
    pub fn new(port : usize) -> Pin {
        Pin::export_pin(port);
        Pin{port:port}
    }
    
    fn export_pin(port : usize) -> bool {
        let fmt_port = &format!("{}", port);
        let export_file_res = File::create("/sys/class/gpio/export");
        
        if File::create("/sys/class/gpio/export").is_ok() {
            export_file_res.unwrap().write_all(fmt_port.as_bytes()).is_ok()
        } else {
            false
        }
    }
    
    fn get_pin_folder(&self) -> String {
        format!("/sys/class/gpio/gpio{}/", self.port)
    }

    pub fn set_mode(&self, direction : Direction) -> bool {
        let direction_file_res = File::create(self.get_pin_folder() + "direction");
        if direction_file_res.is_ok() {
            let mut direction_file = direction_file_res.unwrap();        
            
            let dir = match direction {
                Direction::In => "in",
                Direction::Out => "out"
            };

            direction_file.write_all(dir.as_bytes()).is_ok()
        } else {
            false
        }
    }

    pub fn get_mode(&self) -> Option<Direction> {
        let file_open_res = File::open(self.get_pin_folder() + "direction");
        if file_open_res.is_ok() {        
            let mut direction = String::new();
            let read_result = file_open_res.unwrap().read_to_string(&mut direction);
            
            if read_result.is_ok() {
                match direction.trim() {
                    "in" => Some(Direction::In),
                    "out" => Some(Direction::Out),
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
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
        
        if value_file_create_res.is_ok() {
            let mut value_file = value_file_create_res.unwrap();
            
            let state_str = match state {
                State::High => "1",
                State::Low => "0"
            };

            value_file.write_all(state_str.as_bytes()).is_ok()
        } else {
            false
        }
    }

    pub fn read(&self) -> Option<State> {
        let file_open_res = File::open(self.get_pin_folder() + "value");
        
        if file_open_res.is_ok() {
            let mut pin_value = String::new();
            let read_result = file_open_res.unwrap().read_to_string(&mut pin_value);
            
            match read_result {
                Ok(_) => match pin_value.trim() {
                         "1" => Some(State::High),
                         "0" => Some(State::Low),
                         _ => None
                      },
                Err(_) => None
            }
        } else {
            None
        }
    }
}
