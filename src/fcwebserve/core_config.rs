use iron::prelude::*;
use std::sync::{Arc, Mutex};
use iron::status;
use iron::mime::Mime;
use fccore::Core;

const TAG : &'static str = "webserve_config";

pub fn get_config(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving get config request");
    let json_content_type : Mime = "application/json".parse::<Mime>().unwrap();
    Response::with((json_content_type, status::Ok, core.config().to_string()))
}