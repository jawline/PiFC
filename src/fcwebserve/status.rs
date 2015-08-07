use rustc_serialize::json;
use std::string::{String, ToString};
use fccore::Core;
use std::sync::{Arc, Mutex,MutexGuard};

#[derive(RustcEncodable)]
pub struct Status {
    pub alive: bool,
    pub armed_switch: bool,
    pub armed_cmd: bool,
    pub armed: bool
}

impl Status {
    pub fn from(core: &MutexGuard<Core>) -> Status {
        Status{
            alive: core.alive,
            armed_switch: core.armed_switch(),
            armed_cmd: core.armed_cmd(),
            armed: core.armed()
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        json::encode(self).unwrap()
    }
}