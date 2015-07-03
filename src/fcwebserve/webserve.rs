use iron::prelude::*;
use iron::status;
use fccore::FCCore;
use std::thread;
use std::sync::{Arc, Mutex};

fn unknown() -> IronResult<Response> {
 Ok(Response::with(status::NotFound))
}

fn armed_page(req : &mut Request, core : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
 Ok(Response::with((status::Ok, core.lock().unwrap().armed.to_string())));
}

fn page_handler(req : &mut Request, core : &Arc<Mutex<FCCore>>) -> IronResult<Response> {    	
  println!("Length: {}", req.url.path.len());
  
  if req.url.path.len() == 1 {
   match req.url.path[0] {
    "armed" => armed_page(req, core),
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
