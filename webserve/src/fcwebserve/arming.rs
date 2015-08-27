use iron::prelude::*;
use std::sync::{Arc, Mutex};
use fccore::Core;
use iron::status;

const TAG : &'static str = "webserve_arm";

pub fn arm_core(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.set_armed_command(true);
    Response::with((status::Ok, "arm_cmd set"))
}

pub fn disarm_core(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "disarm core network request");
    core.set_armed_command(false);
    Response::with((status::Ok, "arm_cmd unset"))
}

pub fn kill_core(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.alive = false;
    Response::with((status::Ok, "ok"))
}