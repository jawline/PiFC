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

    pub fn get_mode(&self) -> Direction {
        Direction::Out
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