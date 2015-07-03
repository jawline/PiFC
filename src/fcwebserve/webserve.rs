use iron::prelude::*;
use iron::status;
use fccore::FCCore;
use std::thread;
use std::sync::{Arc, Mutex};

fn landing_page( req : &mut Request ) -> IronResult<Response>  {
 Ok(Response::with((status::Ok, "Landing Page!!")))
}

fn page_handler(req : &mut Request, core : &Arc<Mutex<FCCore>>) -> IronResult<Response> {    	
  println!("Length: {}", req.url.path.len());
  
  if req.url.path.len() == 0 || (req.url.path.len() == 1 && req.url.path[0] == "") {
  	return landing_page(req);
  }
  
  let mut full_path = String::new();	
  
  for partial in req.url.path.iter_mut() {
    println!("Part{}",partial.to_owned());
    full_path = full_path + &partial;
  }
  
  Ok(Response::with((status::Ok, full_path.to_owned())))
}

pub fn spawn(core : &Arc<Mutex<FCCore>>) {

 let webserve_core = core.clone();
 let webserve_addr = "localhost:".to_string() + &core.lock().unwrap().config().fc_webserve_port.to_string();
 
 println!("Spawning WebServe thread");
 
 thread::spawn(move || {
  let webserve_addr_str : &str = &webserve_addr;
  println!("Starting webserve");
  Iron::new(move |req: &mut Request| {
   page_handler(req, &webserve_core)
  }).http(webserve_addr_str).unwrap();
 });
}
