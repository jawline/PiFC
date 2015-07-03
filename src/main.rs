extern crate rustc_serialize;

mod pi;
mod fccore;
mod fcwebserve;

const BASE_CFG_FILE : &'static str = "./assets/base.cfg";

fn main() {
	let (core, handle) = fccore::spawn_fc(BASE_CFG_FILE);

	if core.lock().unwrap().config().fc_webserve_enabled {
		println!("I would start webserve, if I knew how");
	} else {
		println!("WebServe disabled");
	}

	if handle.join().is_err() {
		panic!("Error in FCCore thread");
	}
}
