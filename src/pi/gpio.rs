#![allow(dead_code)]

use std::fs::File;

pub enum Direction {In, Out}

pub struct Pin {
    port : u32,
    fd : File
}

impl Pin {
}