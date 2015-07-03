use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use fccore::FCCore;
use std::thread;
use std::sync::{Arc, Mutex};

fn unknown() -> IronResult<Response> {
 Ok(Response::with((status::NotFound, "unknown command")))
}

fn status_report(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
 let mut core = core_ref.lock().unwrap();
 
 core.log_mut().add("serving status request");
 
 let boiler_start = format!("<html><body>");
 
 let status_portion = format!("ALIVE: {}<br/>", core.alive());
 let arm_portion = format!("ARM_SAFETY: {}<br/>ARM_COMMAND: {}<br/>FULLY ARMED: {}<br/>", core.armed_switch(), core.armed_cmd(), core.armed());
 
 let boiler_end = format!("</body></html>");

 let response = format!("{}{}{}{}", boiler_start, status_portion, arm_portion, boiler_end);

 let html_content_type : Mime = "text/html".parse::<Mime>().unwrap();
 Ok(Response::with((html_content_type, status::Ok, response)))
}

fn get_log(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
 let core = core_ref.lock().unwrap();
 Ok(Response::with((status::Ok, core.log().to_string())))
}

fn arm_core(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
 let mut core = core_ref.lock().unwrap();
 core.log_mut().add("arm core network request");
 core.set_armed_command(true);
 Ok(Response::with((status::Ok, "ok")))
}

fn disarm_core(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
 let mut core = core_ref.lock().unwrap();
 core.log_mut().add("disarm core network request");
 core.set_armed_command(false);
 Ok(Response::with((status::Ok, "ok")))
}

fn page_handler(req : &mut Request, core : &Arc<Mutex<FCCore>>) -> IronResult<Response> {    	
  println!("Length: {}", req.url.path.len());
  
  if req.url.path.len() != 0 {
   let base_cmd : &str = &req.url.path[0].clone();
   match base_cmd {
    "status" => status_report(core),
    "arm" => arm_core(core),
    "disarm" => disarm_core(core),
    "log" => get_log(core),
    _ => unknown()
   }
  } else {
   unknown()
  }
}

pub fn spawn(core : &Arc<Mutex<FCCore>>) {
 let webserve_core = core.clone();
 println!("Spawning WebServe thread");
 thread::spawn(move || {
  let webserve_addr_str : &str = &format!("localhost:{}", webserve_core.lock().unwrap().config().fc_webserve_port);
  println!("Starting webserve on {}", webserve_addr_str);
  Iron::new(move |req: &mut Request| {
   page_handler(req, &webserve_core)
  }).http(webserve_addr_str).unwrap();
 });
}
