use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::path::Path;
use staticfile::Static;
use mount::Mount;
use fccore::Core;
use std::thread;
use std::sync::{Arc, Mutex};
use hyper::header::AccessControlAllowOrigin;
use fccore::log::Lines;
use fcwebserve::config::Config;
use fcwebserve::status::Status;
use fcwebserve::motor_test::motor_test;

const TAG : &'static str = "webserve";

fn unknown() -> Response {
    Response::with((status::NotFound, "unknown command"))
}

fn status_report(core_ref : &Arc<Mutex<Core>>) -> Response {
    let json_content_type : Mime = "application/json".parse::<Mime>().unwrap();
    let core = core_ref.lock().unwrap();
    Response::with((json_content_type, status::Ok, Status::from(&core).to_string()))
}

fn get_log(core_ref : &Arc<Mutex<Core>>) -> Response {
    let core = core_ref.lock().unwrap();
    Response::with((status::Ok, core.log().to_string()))
}

fn get_log_min(core_ref : &Arc<Mutex<Core>>) -> Response {
    let core = core_ref.lock().unwrap();
    Response::with((status::Ok, core.log().to_string_lines_max(Lines::Limit(10))))
}

fn get_config(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving get config request");
    let json_content_type : Mime = "application/json".parse::<Mime>().unwrap();
    Response::with((json_content_type, status::Ok, core.config().to_string()))
}

fn arm_core(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.set_armed_command(true);
    Response::with((status::Ok, "arm_cmd set"))
}

fn disarm_core(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "disarm core network request");
    core.set_armed_command(false);
    Response::with((status::Ok, "arm_cmd unset"))
}

fn kill_core(core_ref : &Arc<Mutex<Core>>) -> Response {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.alive = false;
    Response::with((status::Ok, "ok"))
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
        unknown()
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
