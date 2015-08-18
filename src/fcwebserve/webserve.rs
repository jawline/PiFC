use iron::prelude::*;
use iron::status;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use fccore::Core;
use std::thread;
use std::sync::{Arc, Mutex};
use hyper::header::AccessControlAllowOrigin;
use fcwebserve::config::Config;
use fcwebserve::core_config::get_config;
use fcwebserve::status::status_report;
use fcwebserve::motor_test::motor_test;
use fcwebserve::arming::*;
use fcwebserve::log::*;

const TAG : &'static str = "webserve";

fn unknown(core_ref : &Arc<Mutex<Core>>) -> Response {
    core_ref.lock().unwrap().log_mut().add(TAG, "Unhandled REST request");
    Response::with((status::NotFound, "unknown command"))
}

fn page_handler(req : &mut Request, core : &Arc<Mutex<Core>>) -> IronResult<Response> {    	
    
    let mut full_req_path = String::new();
  
    for item in &req.url.path {
        full_req_path = full_req_path + "/" + item;
    }
    
    let response = if req.url.path.len() != 0 {
        let base_cmd : &str = &req.url.path[0].clone();

        let mut response = match base_cmd {
         "arm" => arm_core(core),
         "disarm" => disarm_core(core),
         "log" => get_log(core),
         "log_reduced" => get_log_min(core),
         "kill" => kill_core(core),
         "config" => get_config(core),
         "motor_test" => motor_test(core),
         "status" | _ => status_report(core)
        };

        response.headers.set(AccessControlAllowOrigin::Any);
        response
    } else {
        unknown(core)
    };

    Ok(response)
}

fn start_webserve_thread(core : Arc<Mutex<Core>>, config: &Config) {
    let webserve_addr = config.api_address.clone();
    let static_addr = config.static_address.clone();
    let static_dir = config.static_dir.clone();
    let alt_core = core.clone();

    //Launch the REST server
    thread::spawn(move || {
        core.lock().unwrap().log_mut().add(TAG, &format!("Starting webserve on {}", webserve_addr));
        Iron::new(move |req: &mut Request| {
            page_handler(req, &core)
        }).http(&webserve_addr as &str).unwrap();
    });

    //Launch the static file server
    thread::spawn(move || {
        alt_core.lock().unwrap().log_mut().add(TAG, &format!("Starting static serve on {} files at {}", &static_addr, &static_dir));
        let mut mount = Mount::new();
        mount.mount("/", Static::new(Path::new(&static_dir)));
        Iron::new(mount).http(&static_addr as &str).unwrap();
    });
}

pub fn spawn(core : &Arc<Mutex<Core>>, config_path: &str) {
    let webserve_config = Config::load(config_path);
    if webserve_config.enabled {
        start_webserve_thread(core.clone(), &webserve_config);
    } else {
        core.lock().unwrap().log_mut().add(TAG, "Webserve disabled by configuration file");
    }
}
