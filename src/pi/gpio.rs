#![allow(dead_code)]

use std::fs::File;

pub enum Direction {In, Out}

pub struct Pin {
    port : usize
}

impl Pin {
    
    pub fn new(port : usize) -> Pin {
        Pin{port:port}
    }

    pub fn get_mode() -> Direction {
        Direction::Out
    }
}