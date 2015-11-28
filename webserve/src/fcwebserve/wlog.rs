use iron::prelude::*;
use simplelog::Lines;
use std::sync::{Arc, Mutex};
use fccore::Core;
use iron::status;

pub fn get_log(core_ref : &Arc<Mutex<Core>>) -> Response {
    let core = core_ref.lock().unwrap();
    Response::with((status::Ok, core.log().to_string()))
}

pub fn get_log_min(core_ref : &Arc<Mutex<Core>>) -> Response {
    let core = core_ref.lock().unwrap();
    Response::with((status::Ok, core.log().to_string_lines_max(Lines::Limit(10))))
}